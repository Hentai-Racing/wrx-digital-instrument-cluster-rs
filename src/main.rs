mod application;
mod can;
mod data;
mod hardware;
mod ui;

use crate::application::s_car_data_bridge::SCarDataBridge;
use crate::can::messages::wrx_2018;
use crate::can::virtual_can_generator::run_vcan_generator;
use crate::data::car_data::CarData;

use socketcan::tokio::CanSocket;
use socketcan::{CanInterface, SocketOptions};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

slint::include_modules!();

const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";
const CAN_BITRATE: u32 = 500000;

fn main() -> Result<(), slint::PlatformError> {
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = tokio_runtime.enter();

    let mut handles = Vec::<tokio::task::JoinHandle<()>>::new();
    let car_data = CarData::new();

    let virtual_cluster = env::var("HR_CLUSTER_VIRTUAL").is_ok_and(|val| val == "1");
    let running_simulation = Arc::new(AtomicBool::new(false));
    let running_vcan = Arc::new(AtomicBool::new(false));
    let mut created_interface = false;

    let (can_if_name, can_if_type) = if virtual_cluster {
        (VCAN_IF_NAME, "vcan")
    } else {
        (CAN_IF_NAME, "can")
    };

    let can_interface: Option<CanInterface> = match CanInterface::open(&can_if_name) {
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
        let mut virtual_socket = CanSocket::open(can_if_name).expect("Failed to open can socket");
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

    // UI
    {
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

        #[cfg(feature = "three_d")]
        {
            use crate::ui::three_d_underlay::ModelContainer;

            let weak_app = ui.as_weak();

            let mut model_container = None;

            ui.window()
                .set_rendering_notifier(move |state, graphics_api| match state {
                    slint::RenderingState::RenderingSetup => {
                        let gl_context = match graphics_api {
                            slint::GraphicsAPI::NativeOpenGL { get_proc_address } => unsafe {
                                three_d::context::Context::from_loader_function_cstr(
                                    get_proc_address,
                                )
                            },
                            _ => return,
                        };

                        if let Ok(context) = three_d::Context::from_gl_context(Arc::new(gl_context))
                        {
                            model_container = Some(ModelContainer::new(context));
                        }
                    }
                    slint::RenderingState::BeforeRendering => {
                        if let (Some(model_container), Some(app)) =
                            (&model_container, weak_app.upgrade())
                        {
                            if app.get_threed_widget_visible() {
                                let image = model_container.render(
                                    app.get_threed_widget_width() as _,
                                    app.get_threed_widget_height() as _,
                                );
                                app.set_threed_widget_texture(image);
                                app.window().request_redraw();
                            }
                        }
                    }
                    _ => {}
                })
                .unwrap();
        }

        ui.run()?;
    }

    running_vcan.store(false, Ordering::SeqCst);
    running_simulation.store(false, Ordering::SeqCst);

    for handle in handles {
        handle.abort();
    }

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
