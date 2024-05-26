slint::include_modules!();

mod can;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use can::can_controller::CanController;
use can::messages::wrx_2018;
use slint::{ComponentHandle, JoinHandle};
use socketcan::{CanFrame, CanInterface, CanSocket, Socket};

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    let vcan_if_name = "vcan0";
    let can_if_name = "can0";

    let mut created_vcan = false;
    let mut can_if_in_use = false;
    let mut in_use_can_if_name: Option<&str> = None;

    let running = Arc::new(AtomicBool::new(true));

    match std::env::var("HR_CLUSTER_VIRTUAL") {
        Ok(_) => match CanInterface::open(vcan_if_name) {
            Ok(_) => {
                created_vcan = true;
                can_if_in_use = true;
                in_use_can_if_name = Some(vcan_if_name);
                println!("Using existing virtual CAN interface");
            }
            _ => match CanInterface::create_vcan(vcan_if_name, Some(0)) {
                Ok(_) => {
                    println!("Created virtual CAN interface {vcan_if_name}");
                    created_vcan = true;
                    can_if_in_use = true;
                    in_use_can_if_name = Some(vcan_if_name);
                }
                _ => println!(
                    "Failed to create virtual CAN interface {vcan_if_name}. Check root privilages"
                ),
            },
        },
        _ => match CanInterface::open(can_if_name) {
            Ok(_) => {
                println!("Using CAN interface {can_if_name}");
                can_if_in_use = true;
                in_use_can_if_name = Some(can_if_name);
            }
            _ => println!("No CAN interface in use"),
        },
    }

    if can_if_in_use {
        if let Some(in_use_can_if_name) = in_use_can_if_name {
            let mut socket_up = false;

            match CanInterface::open(in_use_can_if_name) {
                Ok(can_interface) => match can_interface.bring_up() {
                    Ok(_) => {
                        socket_up = true;
                        println!("Brought up CAN interface {in_use_can_if_name}")
                    }
                    _ => eprintln!("Failed to bring up CAN interface {in_use_can_if_name}"),
                },
                _ => eprintln!("Failed to open CAN interface {in_use_can_if_name}"),
            }

            if socket_up {
                let mut controller = CanController::new(in_use_can_if_name).unwrap();

                let running_clone = running.clone();

                let can_controller_thread = thread::spawn(move || {
                    let ui_clone = ui_weak.clone();
                    while running_clone.load(Ordering::SeqCst) {
                        match controller.read_frame() {
                            Ok(frame) => match frame {
                                CanFrame::Data(data) => {
                                    let data_ref = data.as_ref();

                                    let id = data_ref.can_id;
                                    let payload = data_ref.data;
                                    let _dlc = data_ref.can_dlc;

                                    match wrx_2018::Messages::from_can_message(id, &payload) {
                                        Ok(decoded_message) => match decoded_message {
                                            wrx_2018::Messages::EngineStatus(signal) => {
                                                println!("{:?}", signal.engine_rpm());
                                                ui_clone
                                                    .unwrap()
                                                    .set_engine_rpm(signal.engine_rpm() as i32)
                                            }
                                            _ => {}
                                        },
                                        _ => {}
                                    }
                                }
                                _ => todo!(),
                            },
                            Err(e) => eprintln!("Error reading frame: {}", e),
                        }
                    }
                });

                ui.run()?;

                running.store(false, Ordering::SeqCst);
                can_controller_thread.join().unwrap();
            }

            if created_vcan {
                match CanInterface::open("vcan0") {
                    Ok(vcan_interface) => match vcan_interface.delete() {
                        Ok(_) => println!("Deleted virtual CAN interface {vcan_if_name}"),
                        _ => println!("Failed to delete virtual CAN interface {vcan_if_name}"),
                    },
                    _ => println!("Failed to delete virtual CAN interface {vcan_if_name}"),
                }
            }
        }
    } else {
        ui.run()?;
    }

    Ok(())
}
