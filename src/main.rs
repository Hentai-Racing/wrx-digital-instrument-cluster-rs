mod application;
mod can;
mod data;
mod hardware;
mod ui;

use crate::application::s_car_data_bridge::SCarDataBridge;
use crate::can::can_backend::{CanBackend, CanFrame, SelectedCanInterface};
use crate::can::can_mux_manager::{ISOTPAckFrame, MuxParseResult, OBD2Service};
use crate::data::car_data::{CarData, ParseResult};
use crate::ui::theme_handler;

use std::collections::VecDeque;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

slint::include_modules!();

#[allow(unused)]
const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";

#[cfg(target_os = "linux")]
const DEFAULT_SL_DEV: &str = "/dev/ttyACM0";
#[cfg(target_vendor = "apple")]
const DEFAULT_SL_DEV: &str = "/dev/tty.usbmodem101";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = clap::Command::new("")
        .args([
            clap::arg!(-v --virtual "Runs the application in virtual mode for testing")
                .required(false)
                .exclusive(true),
            clap::arg!(-c --candev <PATH> "Path to the desired CAN device to use")
                .required(false)
                .default_value(CAN_IF_NAME)
                .exclusive(true),
            clap::arg!(-s --sldev <PATH> "Path to the desired serial CAN device to use")
                .required(false)
                .default_value(DEFAULT_SL_DEV)
                .exclusive(true),
        ])
        .get_matches();

    let virtual_cluster = cli.get_flag("virtual");

    // check if the user wants to set anything specific, else default
    let selected_interface = if virtual_cluster {
        SelectedCanInterface::VirtualCan
    } else if cli.value_source("candev") == Some(clap::parser::ValueSource::CommandLine) {
        SelectedCanInterface::Can
    } else if cli.value_source("sldev") == Some(clap::parser::ValueSource::CommandLine) {
        SelectedCanInterface::SerialCan
    } else {
        #[cfg(feature = "apalis_imx8")]
        {
            SelectedCanInterface::Can
        }
        #[cfg(not(feature = "apalis_imx8"))]
        {
            SelectedCanInterface::SerialCan
        }
    };

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = tokio_runtime.enter();

    #[cfg(debug_assertions)]
    if env::var("SLINT_DEBUG_PERFORMANCE")
        .unwrap_or_default()
        .is_empty()
    {
        unsafe {
            env::set_var("SLINT_DEBUG_PERFORMANCE", "refresh_full_speed,overlay");
        }
    }

    let ui = App::new()?;
    ui.show()?;

    let mut handles = Vec::<tokio::task::JoinHandle<()>>::new();
    let mut runners = Vec::<Arc<AtomicBool>>::new();

    let car_data = CarData::new();

    let running_simulation = Arc::new(AtomicBool::new(false));
    let mut _created_interface = false;

    let interface_path = match &selected_interface {
        SelectedCanInterface::SerialCan => {
            if let Some(sldev) = cli.get_one::<String>("sldev") {
                sldev.as_str()
            } else {
                DEFAULT_SL_DEV
            }
        }
        SelectedCanInterface::VirtualCan => VCAN_IF_NAME,
        SelectedCanInterface::Can => {
            if let Some(candev) = cli.get_one::<String>("candev") {
                candev.as_str()
            } else {
                CAN_IF_NAME
            }
        }
    };

    let can_backend = match CanBackend::new(selected_interface, interface_path) {
        Ok(can_backend) => Some(can_backend),
        Err(e) => {
            eprintln!("Error in can backend: {e:?}");
            None
        }
    };
    let running_can = Arc::new(AtomicBool::new(false));

    if let Some(mut can_backend) = can_backend {
        let mut car_data = car_data.clone();
        let running_can = running_can.clone();
        let can_backend_read_handle = tokio_runtime.spawn(async move {
            running_can.store(true, Ordering::SeqCst);

            let obd_id =
                unsafe { embedded_can::Id::from(embedded_can::StandardId::new_unchecked(0x7E0)) };

            // TESTING
            let mut queue = VecDeque::from(vec![
                CanFrame::new(obd_id, 8, &[0x02, OBD2Service::CurrentData.into(), 0x0c]),
                CanFrame::new(
                    obd_id,
                    8,
                    &[0x02, OBD2Service::VehicleInformation.into(), 0x00],
                ),
                CanFrame::new(
                    obd_id,
                    8,
                    &[0x02, OBD2Service::VehicleInformation.into(), 0x02],
                ),
            ]);

            while running_can.load(Ordering::SeqCst) {
                if let Some(frame) = can_backend.read_frame() {
                    match car_data.parse_frame(frame) {
                        Ok(result) => match result {
                            ParseResult::Mux(result) => match result {
                                MuxParseResult::AwaitingBroadcastAck => {
                                    let ack = ISOTPAckFrame::new(obd_id);
                                    queue.push_front(CanFrame::from_frame(ack));
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        Err(e) => println!("Failed to parse frame: {e:?}"),
                    }
                };

                if !car_data.obd_mux_context.waiting_for_responce {
                    if let Some(frame) = queue.pop_front() {
                        match can_backend.write_frame(frame) {
                            Ok(_written_bytes) => {
                                car_data.obd_mux_context.waiting_for_responce = true;
                            }
                            Err(e) => {
                                eprintln!("Failed to write to can_socket: {e:?}");
                            }
                        }
                    }
                }
            }
        });
        handles.push(can_backend_read_handle);

        #[cfg(target_os = "linux")]
        if virtual_cluster {
            use socketcan::{CanSocket, Socket, SocketOptions};

            use crate::can::virtual_can_generator::run_vcan_generator;
            use std::time::Duration;

            let running_vcan = Arc::new(AtomicBool::new(false));

            let mut virtual_socket =
                CanSocket::open(VCAN_IF_NAME).expect("Failed to open can socket");
            // let _ = virtual_socket.set_loopback(true);

            match virtual_socket.set_loopback(true) {
                Err(e) => eprintln!("Failed to set loopback: {e:?}"),
                _ => {}
            }

            running_simulation.store(true, Ordering::SeqCst);
            running_vcan.store(true, Ordering::SeqCst);

            let running_simulation_clone = running_simulation.clone();
            let running_vcan_clone = running_vcan.clone();
            let vcan_handle = tokio_runtime.spawn(async move {
                run_vcan_generator(
                    &mut virtual_socket,
                    running_vcan_clone,
                    running_simulation_clone,
                    Duration::from_millis(1),
                )
            });
            handles.push(vcan_handle);
            runners.push(running_vcan);
        }
    }
    runners.push(running_can);

    let debug_menu_state = ui.global::<DebugMenuState>();
    let application_state = ui.global::<ApplicationState>();

    application_state.set_virtual_cluster(virtual_cluster);
    application_state.set_debug_mode(cfg!(debug_assertions));
    application_state.set_cfg_network(cfg!(feature = "network"));
    application_state.set_cfg_three_d(cfg!(feature = "three-d"));

    let mut ui_data_bridge = SCarDataBridge::new(ui.as_weak(), car_data.clone());
    ui_data_bridge.run();

    if virtual_cluster {
        let running_simulation_clone = running_simulation.clone();
        application_state.on_toggle_simulation(move || {
            running_simulation_clone.store(
                !running_simulation_clone.load(Ordering::SeqCst),
                Ordering::SeqCst,
            );
        });
    }

    #[cfg(feature = "apalis_imx8")]
    {
        use crate::hardware::apalis_imx8::ApalisIMX8;

        let device = ApalisIMX8::new();

        debug_menu_state.on_debug_suspend(move || {
            device.power_suspend();
        });
    }
    #[cfg(not(feature = "apalis_imx8"))]
    {
        debug_menu_state.on_debug_suspend(|| println!("DEBUG: DO SUSPEND"));
    }

    #[cfg(feature = "three-d")]
    {
        use crate::ui::three_d_underlay::ModelContainer;

        let weak_app = ui.as_weak();
        let mut model_container: Option<ModelContainer> = None;
        let render_notifier = ui
            .window()
            .set_rendering_notifier(move |state, graphics_api| match state {
                slint::RenderingState::RenderingSetup => {
                    let gl_context = match graphics_api {
                        slint::GraphicsAPI::NativeOpenGL { get_proc_address } => unsafe {
                            three_d::context::Context::from_loader_function_cstr(get_proc_address)
                        },
                        _ => return,
                    };

                    if let (Ok(context), Some(app)) = (
                        three_d::Context::from_gl_context(Arc::new(gl_context)),
                        weak_app.upgrade(),
                    ) {
                        model_container = Some(ModelContainer::new(
                            context, 0, 0, 1,
                            1, // app.get_threed_widget_x() as _,
                              // app.get_threed_widget_y() as _,
                              // app.get_threed_widget_width() as _,
                              // app.get_threed_widget_height() as _,
                        ));
                    }
                }
                slint::RenderingState::BeforeRendering => {
                    if let (Some(model_container), Some(app)) =
                        (&mut model_container, weak_app.upgrade())
                    {
                        if app.get_threed_widget_visible() {
                            let (width, height) =
                                (app.window().size().width, app.window().size().height);

                            let image = model_container.render(
                                0, 0, 1,
                                1, // app.get_threed_widget_x() as _,
                                  // app.get_threed_widget_y() as _,
                                  // app.get_threed_widget_width() as _,
                                  // app.get_threed_widget_height() as _,
                            );

                            // app.set_threed_widget_texture(image);
                            app.window().request_redraw();
                        }
                    }
                }
                _ => {}
            });

        if let Err(e) = render_notifier {
            println!("Error setting rendering notifier: {e:?}");
        }
    }

    runners.push(running_simulation);

    #[cfg(feature = "network")]
    let _navmap = crate::ui::navmap::NavMap::new(ui.as_weak());

    // main loop

    let weak_ui = ui.as_weak();
    theme_handler::handle_theme(weak_ui);

    ui.run()?;

    // cleanup

    for runner in runners {
        runner.store(false, Ordering::SeqCst);
    }

    for handle in handles {
        handle.abort();
    }

    // TODO: figure out why we aren't cleaning up properly

    // tokio_runtime.block_on(async {
    //     join_all(handles).await;
    // });

    // if can_backend.created_interface() {
    //     match can_backend.delete() {
    //         // TODO: these should be the name of the can interface, not VCAN_IF_NAME
    //         Ok(_) => println!("Deleted interface {VCAN_IF_NAME}"),
    //         Err(e) => println!("Error deleting interface {VCAN_IF_NAME}: {e:?}"),
    //     }
    // }

    Ok(())
}
