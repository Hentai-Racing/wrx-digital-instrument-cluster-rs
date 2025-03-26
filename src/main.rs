mod application;
mod can;
mod data;
mod hardware;
mod ui;

use crate::application::s_car_data_bridge::SCarDataBridge;
use crate::data::car_data::CarData;

use std::env;
use std::path::Path;
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
                .default_value(CAN_IF_NAME)
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

    #[derive(PartialEq, Eq)]
    enum SelectedInterface {
        Virtual,
        Can,
        SerialCan,
    }

    // check if the user wants to set anything specific, else default
    let selected_interface = if virtual_cluster {
        SelectedInterface::Virtual
    } else if matches.value_source("candev") == Some(clap::parser::ValueSource::CommandLine) {
        SelectedInterface::Can
    } else if matches.value_source("sldev") == Some(clap::parser::ValueSource::CommandLine) {
        SelectedInterface::SerialCan
    } else {
        #[cfg(target_os = "linux")]
        {
            SelectedInterface::Can
        }
        #[cfg(not(target_os = "linux"))]
        {
            SelectedInterface::SerialCan
        }
    };

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = tokio_runtime.enter();

    let mut handles = Vec::<tokio::task::JoinHandle<()>>::new();
    let mut runners = Vec::<Arc<AtomicBool>>::new();

    let car_data = CarData::new();

    let running_simulation = Arc::new(AtomicBool::new(false));
    let running_vcan = Arc::new(AtomicBool::new(false));
    let mut created_interface = false;

    #[cfg(target_os = "linux")]
    let mut can_interface = None;

    #[cfg(target_os = "linux")]
    {
        use socketcan::tokio::CanSocket;
        use socketcan::{CanInterface, SocketOptions};

        let (can_if_name, can_if_type) = if virtual_cluster {
            (VCAN_IF_NAME, "vcan")
        } else {
            (
                if let Some(candev) = candev {
                    candev.as_str()
                } else {
                    CAN_IF_NAME
                },
                "can",
            )
        };

        if matches!(
            selected_interface,
            SelectedInterface::Virtual | SelectedInterface::Can
        ) {
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

    let debug_menu_state = ui.global::<DebugMenuState>();
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

    if selected_interface == SelectedInterface::SerialCan {
        if let Some(sldev) = sldev {
            match serial::SystemPort::open(Path::new(sldev)) {
                Ok(port) => slport = Some(port),
                Err(e) => eprintln!("Error opening serial device {sldev}: {e}"),
            }
        }
    }

    let running_slcan_clone = running_sclan.clone();
    let mut car_data_clone = car_data.clone();
    let slport_handle = tokio_runtime.spawn(async move {
        if let Some(slport) = slport {
            running_slcan_clone.store(true, Ordering::SeqCst);

            let mut can_socket = slcan::CanSocket::<serial::SystemPort>::new(slport);
            can_socket.close().unwrap();
            can_socket.open(slcan::BitRate::Setup1Mbit).unwrap();

            car_data_clone
                .bridge_slcan(can_socket, running_slcan_clone)
                .await;
        }
    });
    handles.push(slport_handle);

    runners.push(running_sclan);
    runners.push(running_vcan);
    runners.push(running_simulation);

    // main loop

    ui.run()?;

    // cleanup

    for runner in runners {
        runner.store(false, Ordering::SeqCst);
    }

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
