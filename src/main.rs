mod application;
mod can;
mod data;
mod hardware;
mod ui;

use crate::application::s_car_data_bridge::SCarDataBridge;
use crate::data::car_data::CarData;

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

slint::include_modules!();

const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";
const CAN_BITRATE: u32 = 500000;

#[cfg(target_os = "linux")]
const DEFAULT_SL_DEV: &str = "/dev/ttyACM0";
#[cfg(target_vendor = "apple")]
const DEFAULT_SL_DEV: &str = "/dev/tty.usbmodem101";

fn main() -> Result<(), slint::PlatformError> {
    let matches = clap::Command::new("")
        .args([
            clap::arg!(-v --virtual "Runs the application in virtual mode for testing")
                .required(false)
                .exclusive(true),
            clap::arg!(-c --candev <PATH> "Path to the desired CAN device to use")
                .required(false)
                .exclusive(true),
            clap::arg!(-s --sldev <PATH> "Path to the desired serial CAN device to use")
                .required(false)
                .default_value(DEFAULT_SL_DEV)
                .exclusive(true),
        ])
        .get_matches();

    let virtual_cluster = matches.get_flag("virtual");
    let candev = matches.get_one::<String>("candev");
    let sldev = matches.get_one::<String>("sldev");

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = tokio_runtime.enter();

    let mut handles = Vec::<tokio::task::JoinHandle<()>>::new();

    let car_data = CarData::new();

    let running_simulation = Arc::new(AtomicBool::new(false));
    let running_vcan = Arc::new(AtomicBool::new(false));
    let mut created_interface = false;

    let (can_if_name, can_if_type) = if virtual_cluster {
        (VCAN_IF_NAME, "vcan")
    } else {
        (CAN_IF_NAME, "can")
    };

    #[cfg(target_os = "linux")]
    {
        use socketcan::tokio::CanSocket;
        use socketcan::{CanInterface, SocketOptions};

        let can_interface = None;
        if sldev.is_none() {
            can_interface = match CanInterface::open(&can_if_name) {
                Ok(can_interface) => Some(can_interface),
                _ => match CanInterface::create(&can_if_name, None, can_if_type) {
                    Ok(can_interface) => {
                        created_interface = true;
                        println!("Created CAN interface {can_if_name}");
                        Some(can_interface)
                    }
                    Err(e) => {
                        eprintln!("Failed to create CAN interface {can_if_name}: {e}");
                        None
                    }
                },
            };
        }

        let socket_up = if let Some(can_interface) = &can_interface {
            match can_interface.details() {
                Ok(details) => {
                    if details.is_up {
                        true
                    } else {
                        if can_if_type != "vcan" {
                            match can_interface.set_bitrate(CAN_BITRATE, None) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!(
                                    "Failed to set bitrate of {can_if_name} to {CAN_BITRATE}: {e:?}"
                                );
                                }
                            };
                        }

                        match can_interface.bring_up() {
                            Ok(_) => true,
                            Err(e) => {
                                eprintln!("Failed to bring up interface {can_if_name}: {e:?}");
                                false
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get details from interface {can_if_name}: {e:?}");
                    false
                }
            }
        } else {
            false
        };

        if socket_up {
            let can_socket = CanSocket::open(can_if_name).expect("Failed to open can socket");

            let mut car_data_clone = car_data.clone();

            let car_data_bridge_handle = tokio_runtime.spawn(async move {
                car_data_clone.bridge_socketcan(can_socket).await;
            });
            handles.push(car_data_bridge_handle);
        }

        if virtual_cluster {
            use crate::can::virtual_can_generator::run_vcan_generator;
            use std::time::Duration;

            let mut virtual_socket =
                CanSocket::open(can_if_name).expect("Failed to open can socket");
            let _ = virtual_socket.set_loopback(true);

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
                .await
            });
            handles.push(vcan_handle);
        }
    }

    #[cfg(debug_assertions)]
    if env::var("SLINT_DEBUG_PERFORMANCE")
        .unwrap_or_default()
        .is_empty()
    {
        env::set_var("SLINT_DEBUG_PERFORMANCE", "refresh_full_speed,overlay");
    }

    let ui = App::new()?;

    let application_state = ui.global::<ApplicationState>();
    application_state.set_virtual_cluster(virtual_cluster);
    application_state.set_debug_mode(cfg!(debug_assertions));

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

    let debug_menu_state = ui.global::<DebugMenuState>();

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
                            context,
                            app.get_threed_widget_x() as _,
                            app.get_threed_widget_y() as _,
                            app.get_threed_widget_width() as _,
                            app.get_threed_widget_height() as _,
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
                                app.get_threed_widget_x() as _,
                                app.get_threed_widget_y() as _,
                                app.get_threed_widget_width() as _,
                                app.get_threed_widget_height() as _,
                            );

                            app.set_threed_widget_texture(image);
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

    let running_sclan = Arc::new(AtomicBool::new(false));
    let mut slport = None;

    // todo: ensure we are not using a can device or virtual
    if candev.is_none() && !virtual_cluster {
        if let Some(sldev) = sldev {
            match serial::SystemPort::open(std::path::Path::new(sldev)) {
                Ok(t) => slport = Some(t),
                Err(e) => eprintln!("Error opening serial device {sldev}: {e}"),
            }
        }
    }

    async fn run(
        mut can: slcan::CanSocket<serial::unix::TTYPort>,
        mut car_data: CarData,
        running: Arc<AtomicBool>,
    ) {
        // todo: move this to car_data to be inline with current implementation of socketcan
        use crate::can::messages::wrx_2018::Messages;
        use embedded_can::{Id, StandardId};

        while running.load(Ordering::SeqCst) {
            match can.read() {
                Ok(frame) => {
                    let id = unsafe { Id::from(StandardId::new_unchecked(frame.id as _)) };
                    if let Ok(message) = Messages::from_can_message(id, &frame.data[..frame.dlc]) {
                        car_data.process_message(&message);
                    }
                }
                #[cfg(debug_assertions)]
                Err(e) => eprintln!("Error reading frame: {e:?}"),
                #[allow(unreachable_patterns)]
                _ => {}
            }
        }
    }

    let running_slcan_clone = running_sclan.clone();
    let slport_handle = tokio_runtime.spawn(async move {
        if let Some(slport) = slport {
            running_slcan_clone.store(true, Ordering::SeqCst);

            let mut can = slcan::CanSocket::<serial::SystemPort>::new(slport);
            can.close().unwrap();
            can.open(slcan::BitRate::Setup1Mbit).unwrap();

            run(can, car_data, running_slcan_clone).await;
        }
    });
    handles.push(slport_handle);

    ui.run()?;

    running_sclan.store(false, Ordering::SeqCst);
    running_vcan.store(false, Ordering::SeqCst);
    running_simulation.store(false, Ordering::SeqCst);

    for handle in handles {
        handle.abort();
    }

    #[cfg(target_os = "linux")]
    if created_interface {
        if let Some(can_interface) = can_interface {
            match can_interface.delete() {
                Ok(_) => println!("Deleted interface {VCAN_IF_NAME}"),
                Err(e) => println!("Error deleting interface {VCAN_IF_NAME}: {e:?}"),
            }
        }
    }

    Ok(())
}
