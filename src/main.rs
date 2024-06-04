slint::include_modules!();

mod can;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use can::can_controller::CanController;
use can::messages::wrx_2018;

use rand::Rng;
use slint::{ComponentHandle, Weak};
use socketcan::{CanDataFrame, CanFrame, CanInterface, CanSocket, Frame, Socket};

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    let vcan_if_name = "vcan0";
    let can_if_name = "can0";

    let mut virtual_cluster = false;
    let mut created_vcan = false;
    let mut in_use_can_if_name: Option<&str> = None;

    let running = Arc::new(AtomicBool::new(true));

    match std::env::var("HR_CLUSTER_VIRTUAL") {
        Ok(val) => virtual_cluster = val == "1",
        _ => {}
    }

    if virtual_cluster {
        match CanInterface::open(vcan_if_name) {
            Ok(_) => {
                in_use_can_if_name = Some(vcan_if_name);
                println!("Using existing virtual CAN interface");
            }
            _ => match CanInterface::create_vcan(vcan_if_name, Some(0)) {
                Ok(_) => {
                    println!("Created virtual CAN interface {vcan_if_name}");
                    created_vcan = true;
                    in_use_can_if_name = Some(vcan_if_name);
                }
                _ => println!(
                    "Failed to create virtual CAN interface {vcan_if_name}. Check privilages"
                ),
            },
        }
    } else {
        match CanInterface::open(can_if_name) {
            Ok(_) => {
                println!("Using CAN interface {can_if_name}");
                in_use_can_if_name = Some(can_if_name);
            }
            _ => println!("No CAN interface in use"),
        }
    }

    if let Some(in_use_can_if_name) = in_use_can_if_name {
        let mut socket_up = false;

        match CanInterface::open(in_use_can_if_name) {
            Ok(can_interface) => match can_interface.details() {
                Ok(details) => {
                    if details.is_up {
                        socket_up = true;
                        println!("CAN interface {in_use_can_if_name} is already up. Continuing...")
                    } else {
                        match can_interface.bring_up() {
                            Ok(_) => {
                                socket_up = true;
                                println!("Brought up CAN interface {in_use_can_if_name}")
                            }
                            _ => eprintln!("Failed to bring up CAN interface {in_use_can_if_name}"),
                        }
                    }
                }
                _ => {}
            },
            _ => eprintln!("Failed to open CAN interface {in_use_can_if_name}"),
        }

        if socket_up {
            let mut controller = CanController::new(in_use_can_if_name).unwrap();

            let running_clone = running.clone();

            let can_controller_thread = thread::spawn(move || {
                while running_clone.load(Ordering::SeqCst) {
                    match controller.read_frame() {
                        Ok(frame) => match frame {
                            CanFrame::Data(data_frame) => {
                                parse_can_frame(ui_weak.clone(), data_frame)
                            }
                            _ => todo!(),
                        },
                        Err(e) => eprintln!("Error reading frame: {e:?}"),
                    }
                }
            });

            let mut virtual_handler_thread: Option<std::thread::JoinHandle<()>> = None;

            if virtual_cluster {
                match handle_virtual_can(vcan_if_name, running.clone()) {
                    Ok(handle) => {
                        virtual_handler_thread = Some(handle);
                    }
                    Err(e) => eprintln!("{e:?}"),
                }
            }

            ui.run()?;

            running.store(false, Ordering::SeqCst);
            can_controller_thread.join().unwrap();

            if let Some(virtual_handler_handle) = virtual_handler_thread {
                virtual_handler_handle.join().unwrap();
            }
        }

        if created_vcan {
            match CanInterface::open(vcan_if_name) {
                Ok(vcan_interface) => match vcan_interface.delete() {
                    Ok(_) => println!("Deleted virtual CAN interface {vcan_if_name}"),
                    _ => println!("Failed to delete virtual CAN interface {vcan_if_name}"),
                },
                _ => println!("Failed to delete virtual CAN interface {vcan_if_name}"),
            }
        }
    } else {
        println!("Showing stale UI");

        ui.run()?;
    }

    Ok(())
}

fn parse_can_frame(ui: Weak<AppWindow>, frame: CanDataFrame) {
    let frame_ref = frame.as_ref();

    let id = frame_ref.can_id;
    let payload = frame_ref.data;
    let _dlc = frame_ref.can_dlc;

    match wrx_2018::Messages::from_can_message(id, &payload) {
        Ok(decoded_message) => match decoded_message {
            wrx_2018::Messages::EngineStatus(signal) => {
                slint::invoke_from_event_loop(move || {
                    ui.unwrap().set_engine_rpm(DataParameter {
                        min_value: wrx_2018::EngineStatus::ENGINE_RPM_MIN as i32,
                        max_value: wrx_2018::EngineStatus::ENGINE_RPM_MAX as i32,
                        value: signal.engine_rpm() as i32,
                    })
                })
                .unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

fn handle_virtual_can(
    vcan_if_name: &str,
    running: Arc<AtomicBool>,
) -> Result<std::thread::JoinHandle<()>, ()> {
    let socket = CanSocket::open(vcan_if_name);
    match socket {
        Ok(socket) => Ok(thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let rpm = rand::thread_rng().gen_range(
                    wrx_2018::EngineStatus::ENGINE_RPM_MIN..wrx_2018::EngineStatus::ENGINE_RPM_MAX,
                );
                match wrx_2018::EngineStatus::new(0, true, 0, rpm, 0) {
                    Ok(dbc_frame) => {
                        let message_id = wrx_2018::EngineStatus::MESSAGE_ID;
                        let frame = CanDataFrame::from_raw_id(message_id, dbc_frame.raw());
                        if let Some(frame) = frame {
                            socket.write_frame::<CanDataFrame>(&frame).unwrap();
                        }
                    }
                    _ => {}
                }
            }
        })),
        _ => Err(eprintln!("Failed to run virtual interface {vcan_if_name}")),
    }
}
