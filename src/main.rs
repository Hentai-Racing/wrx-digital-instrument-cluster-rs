slint::include_modules!();

mod can;
mod unit_conversion;
use crate::can::messages::wrx_2018;
use crate::can::virtual_can_generator::handle_virtual_can;
use embedded_can;
use slint::{ComponentHandle, Weak};
use socketcan::{CanFrame, CanInterface, CanSocket, Socket};
use std::env;
use std::string::ToString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use unit_conversion::Units;

const IMPL_SAVE_STATE: bool = false; // todo: implement save state
const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    let virtual_cluster = env::var("HR_CLUSTER_VIRTUAL").is_ok_and(|val| val == "1");
    let mut created_vcan = false;
    let mut in_use_can_if_name: Option<&str> = None;

    let running = Arc::new(AtomicBool::new(true));

    let loaded_save_state = IMPL_SAVE_STATE;

    if virtual_cluster {
        in_use_can_if_name = CanInterface::open(&VCAN_IF_NAME)
            .is_ok()
            .then_some(VCAN_IF_NAME)
            .inspect(|vcan_if_name| println!("Using virtual CAN interface {vcan_if_name}"))
            .or_else(|| {
                CanInterface::create_vcan(&VCAN_IF_NAME, Some(0))
                    .is_ok()
                    .then(|| {
                        created_vcan = true;
                        VCAN_IF_NAME
                    })
            })
            .inspect(|vcan_if_name| println!("Created virtual CAN interface {vcan_if_name}"));
        in_use_can_if_name.expect("Failed to create virtual CAN interface");
    } else {
        in_use_can_if_name = CanInterface::open(&CAN_IF_NAME)
            .is_ok()
            .then_some(CAN_IF_NAME);
    }

    if loaded_save_state {
        todo!();
    } else {
        ui_weak
            .unwrap()
            .global::<CarData>()
            .set_units(Units::USCS.into());
    }

    if let Some(can_if_name) = in_use_can_if_name {
        let socket_up = CanInterface::open(can_if_name)
            .and_then(|can_interface| {
                Ok(can_interface.details().is_ok_and(|details| {
                    if details.is_up {
                        println!("CAN interface {can_if_name} is already up. Continuing...");
                        true
                    } else {
                        can_interface
                            .bring_up()
                            .and_then(|_| Ok(true))
                            .expect("Failed to bring up CAN interface")
                    }
                }))
            })
            .expect("Failed to open CAN interface");

        if socket_up {
            let socket = CanSocket::open(can_if_name).expect("Failed to open CAN socket");

            socket
                .set_nonblocking(true)
                .expect("Failed to set non-blocking mode");

            socket
                .set_read_timeout(std::time::Duration::from_millis(100))
                .expect("Failed to set read timeout");

            socket
                .set_write_timeout(std::time::Duration::from_millis(100))
                .expect("Failed to set write timeout");

            let running_clone = running.clone();

            // todo: implement async reading
            let can_controller_thread = thread::spawn(move || {
                while running_clone.load(Ordering::SeqCst) {
                    match socket.read_frame() {
                        Ok(frame) => match frame {
                            CanFrame::Data(data_frame) => {
                                parse_can_frame(ui_weak.clone(), data_frame)
                            }
                            _ => {
                                println!("Received non-data frame [not yet implemented]: {frame:?}")
                            }
                        },
                        Err(e) => eprintln!("Error reading frame: {e:?}"),
                    }
                }
            });

            let mut virtual_handler_thread: Option<std::thread::JoinHandle<()>> = None;

            if virtual_cluster {
                match handle_virtual_can(VCAN_IF_NAME, running.clone()) {
                    Ok(handle) => {
                        virtual_handler_thread = Some(handle);
                        println!("Started virtual CAN handler");
                    }
                    Err(e) => eprintln!("{e:?}"),
                }
            }

            ui.run()?;

            running.store(false, Ordering::SeqCst);
            can_controller_thread.join().unwrap();

            if let Some(virtual_handler_handle) = virtual_handler_thread {
                virtual_handler_handle
                    .join()
                    .expect("Failed to join virtual handler thread");
            }
        }

        if created_vcan {
            match CanInterface::open(VCAN_IF_NAME) {
                Ok(vcan_interface) => match vcan_interface.delete() {
                    Ok(_) => println!("Deleted virtual CAN interface {VCAN_IF_NAME}"),
                    _ => println!("Failed to delete virtual CAN interface {VCAN_IF_NAME}"),
                },
                _ => println!("Failed to delete virtual CAN interface {VCAN_IF_NAME}"),
            }
        }
    } else {
        println!("Showing stale UI");

        ui.run()?;
    }

    Ok(())
}

fn parse_can_frame(ui: Weak<AppWindow>, frame: impl embedded_can::Frame) {
    use wrx_2018::Messages;

    match Messages::from_can_message(frame.id(), frame.data()) {
        Ok(decoded_message) => match decoded_message {
            Messages::EngineStatus(signal) => {
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
            Messages::XxxMsg209(signal) => {
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
            Messages::Odometer(signal) => {
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
            Messages::StatusSwitches(signal) => slint::invoke_from_event_loop(move || {
                let binding = ui.unwrap();
                let cardata = binding.global::<CarData>();
                cardata.set_lowbeams_enabled(signal.lowbeams_enabled())
            })
            .unwrap(),
            Messages::XxxMsg640(signal) => slint::invoke_from_event_loop(move || {
                let binding = ui.unwrap();
                let cardata = binding.global::<CarData>();

                cardata.set_left_turn_signal_enabled(signal.left_turn_signal_enabled());
                cardata.set_right_turn_signal_enabled(signal.right_turn_signal_enabled());
            })
            .unwrap(),
            _ => {}
        },
        _ => {}
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

// implement these here because Slint is included here
impl Into<SUnits> for Units {
    fn into(self) -> SUnits {
        match self {
            Units::SI => SUnits::SI,
            Units::USCS => SUnits::USCS,
        }
    }
}

impl From<SUnits> for Units {
    fn from(units: SUnits) -> Self {
        match units {
            SUnits::SI => Units::SI,
            SUnits::USCS => Units::USCS,
        }
    }
}
