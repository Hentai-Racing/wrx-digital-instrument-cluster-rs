use crate::can::messages::wrx_2018;
use embedded_can::Frame;
use rand::Rng;
use socketcan::{CanDataFrame, CanSocket, Socket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub fn handle_virtual_can(
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
                        if let Some(frame) = CanDataFrame::new(dbc_frame.id(), dbc_frame.data()) {
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
                        if let Some(frame) = CanDataFrame::new(dbc_frame.id(), dbc_frame.data()) {
                            socket.write_frame::<CanDataFrame>(&frame).unwrap();
                        }
                    }
                    _ => {}
                };

                let odometer = rand::thread_rng()
                    .gen_range(wrx_2018::Odometer::ODOMETER_MIN..wrx_2018::Odometer::ODOMETER_MAX);
                match wrx_2018::Odometer::new(odometer) {
                    Ok(dbc_frame) => {
                        if let Some(frame) = CanDataFrame::new(dbc_frame.id(), dbc_frame.data()) {
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
                        let frame = CanDataFrame::new(message_id, dbc_frame.raw());
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
