slint::include_modules!();

mod can;
mod unit_conversion;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use can::can_controller::CanController;
use can::messages::wrx_2018;
use unit_conversion::{PressureUnits, UnitHandler, Units};

use rand::Rng;
use slint::{ComponentHandle, Weak};
use socketcan::{CanDataFrame, CanFrame, CanInterface, CanSocket, Frame, Socket};

use std::string::ToString;

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
            controller
                .set_timeout(std::time::Duration::from_millis(100))
                .unwrap();
            // let unit_handler = UnitHandler::new(Units::SI, Units::UCS);

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

    match wrx_2018::Messages::from_can_message(id, &payload[..frame_ref.can_dlc.into()]) {
        Ok(decoded_message) => match decoded_message {
            wrx_2018::Messages::EngineStatus(signal) => {
                slint::invoke_from_event_loop(move || {
                    let binding = ui.unwrap();
                    let cardata = binding.global::<CarData>();

                    cardata.set_engine_rpm(IDataParameter {
                        min_value: wrx_2018::EngineStatus::ENGINE_RPM_MIN.into(),
                        max_value: wrx_2018::EngineStatus::ENGINE_RPM_MAX.into(),
                        value: signal.engine_rpm().into(),
                        units: "rpm".into(),
                    });

                    cardata.set_mt_gear(MTGearParameter {
                        value: signal.mt_gear_raw().into(),
                        display: signal.mt_gear().to_string().into(),
                    });
                })
                .unwrap();
            }
            wrx_2018::Messages::XxxMsg209(signal) => {
                slint::invoke_from_event_loop(move || {
                    let binding = ui.unwrap();
                    let cardata = binding.global::<CarData>();
                    cardata.set_vehicle_speed(FDataParameter {
                        min_value: unit_conversion::kph_to_mph(
                            wrx_2018::XxxMsg209::VEHICLE_SPEED_MIN,
                        ),
                        max_value: unit_conversion::kph_to_mph(
                            wrx_2018::XxxMsg209::VEHICLE_SPEED_MAX,
                        ),
                        value: unit_conversion::kph_to_mph(signal.vehicle_speed()),
                        units: "mph".into(),
                    })
                })
                .unwrap();
            }
            wrx_2018::Messages::Odometer(signal) => {
                slint::invoke_from_event_loop(move || {
                    let binding = ui.unwrap();
                    let cardata = binding.global::<CarData>();
                    cardata.set_odometer(FDataParameter {
                        min_value: wrx_2018::Odometer::ODOMETER_MIN,
                        max_value: wrx_2018::Odometer::ODOMETER_MAX,
                        value: signal.odometer(),
                        units: "mi".into(),
                    })
                })
                .unwrap();
            }
            wrx_2018::Messages::StatusSwitches(signal) => {
                slint::invoke_from_event_loop(move || {
                    let binding = ui.unwrap();
                    let cardata = binding.global::<CarData>();
                    cardata.set_lowbeams_enabled(signal.lowbeams_enabled())
                })
                .unwrap()
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
                    wrx_2018::EngineStatus::ENGINE_RPM_MIN..=wrx_2018::EngineStatus::ENGINE_RPM_MAX,
                );
                let mt_gear = rand::thread_rng().gen_range(
                    wrx_2018::EngineStatus::MT_GEAR_MIN..=wrx_2018::EngineStatus::MT_GEAR_MAX,
                );

                match wrx_2018::EngineStatus::new(0, true, 0, rpm, mt_gear) {
                    Ok(dbc_frame) => {
                        let message_id = wrx_2018::EngineStatus::MESSAGE_ID;
                        let frame = CanDataFrame::from_raw_id(message_id, dbc_frame.raw());
                        if let Some(frame) = frame {
                            socket.write_frame::<CanDataFrame>(&frame).unwrap();
                        }
                    }
                    _ => {}
                };

                let speed = rand::thread_rng().gen_range(
                    wrx_2018::XxxMsg209::VEHICLE_SPEED_MIN..wrx_2018::XxxMsg209::VEHICLE_SPEED_MAX,
                );
                match wrx_2018::XxxMsg209::new(speed, 0.0) {
                    Ok(dbc_frame) => {
                        let message_id = wrx_2018::XxxMsg209::MESSAGE_ID;
                        let frame = CanDataFrame::from_raw_id(message_id, dbc_frame.raw());
                        if let Some(frame) = frame {
                            socket.write_frame::<CanDataFrame>(&frame).unwrap();
                        }
                    }
                    _ => {}
                };

                let odometer = rand::thread_rng()
                    .gen_range(wrx_2018::Odometer::ODOMETER_MIN..wrx_2018::Odometer::ODOMETER_MAX);
                match wrx_2018::Odometer::new(odometer) {
                    Ok(dbc_frame) => {
                        let message_id = wrx_2018::Odometer::MESSAGE_ID;
                        let frame = CanDataFrame::from_raw_id(message_id, dbc_frame.raw());
                        if let Some(frame) = frame {
                            socket.write_frame::<CanDataFrame>(&frame).unwrap();
                        }
                    }
                    _ => {}
                };

                let lowbeams_enabled = rand::random::<bool>();
                match wrx_2018::StatusSwitches::new(
                    false,
                    false,
                    false,
                    false,
                    false,
                    lowbeams_enabled,
                    false,
                    false,
                ) {
                    Ok(dbc_frame) => {
                        let message_id = wrx_2018::StatusSwitches::MESSAGE_ID;
                        let frame = CanDataFrame::from_raw_id(message_id, dbc_frame.raw());
                        if let Some(frame) = frame {
                            socket.write_frame::<CanDataFrame>(&frame).unwrap();
                        }
                    }
                    _ => {}
                };
            }
        })),
        _ => Err(eprintln!("Failed to run virtual interface {vcan_if_name}")),
    }
}

impl ToString for wrx_2018::EngineStatusMtGear {
    fn to_string(&self) -> String {
        match self {
            wrx_2018::EngineStatusMtGear::Floating => " ".to_string(),
            wrx_2018::EngineStatusMtGear::Neutral => "N".to_string(),
            wrx_2018::EngineStatusMtGear::X1 => "1".to_string(),
            wrx_2018::EngineStatusMtGear::X2 => "2".to_string(),
            wrx_2018::EngineStatusMtGear::X3 => "3".to_string(),
            wrx_2018::EngineStatusMtGear::X4 => "4".to_string(),
            wrx_2018::EngineStatusMtGear::X5 => "5".to_string(),
            wrx_2018::EngineStatusMtGear::X6 => "6".to_string(),
            _ => "?ERR_MT_GEAR".to_string(),
        }
    }
}
