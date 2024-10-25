mod application;
mod can;
mod data;

use crate::application::can_data_bridge::CanDataBridge;
use crate::application::ui_data_bridge::UIDataBridge;
use crate::can::messages::wrx_2018;
use crate::can::virtual_can_generator::run_vcan_generator;
use crate::data::car_data::CarData;
use crate::data::units::UnitSystem;

use socketcan::tokio::CanSocket;
use socketcan::{CanInterface, SocketOptions};
use std::env;
use std::string::ToString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;

const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = tokio_runtime.enter();

    let mut handles = Vec::<tokio::task::JoinHandle<()>>::new();

    let mut car_data = CarData::new();

    let virtual_cluster = env::var("HR_CLUSTER_VIRTUAL").is_ok_and(|val| val == "1");
    let running_vcan = Arc::new(AtomicBool::new(false));
    let mut created_interface = false;

    let init_ui = true;

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

        let mut can_data_bridge = CanDataBridge::new(car_data.clone(), can_socket);

        let can_bridge_handle = tokio_runtime.spawn(async move {
            can_data_bridge.read_can_frames().await;
        });
        handles.push(can_bridge_handle);
    }

    if virtual_cluster {
        let mut virtual_socket = CanSocket::open(can_if_name).expect("Failed to open can socket");
        let _ = virtual_socket.set_loopback(true);

        running_vcan.store(true, Ordering::SeqCst);

        let running_vcan_clone = running_vcan.clone();
        let vcan_handle = tokio_runtime.spawn(async move {
            run_vcan_generator(&mut virtual_socket, running_vcan_clone).await
        });
        handles.push(vcan_handle);
    }

    if init_ui {
        let ui = AppWindow::new()?;

        let mut ui_data_bridge = UIDataBridge::new(ui.as_weak(), car_data.clone());
        ui_data_bridge.run();

        ui.run()?
    } else {
        tokio_runtime.spawn(async move {
            let mut engine_rpm = car_data.engine_rpm().watch();

            loop {
                if engine_rpm.changed().await.is_ok() {
                    println!(
                        "Engine RPM changed: {}",
                        engine_rpm.borrow_and_update().clone()
                    )
                }
            }
        });

        println!("Ctrl+C to stop");
        tokio_runtime.block_on(async {
            signal::ctrl_c()
                .await
                .expect("Failed to listen for ctrl_c signal");
        });
        println!();
    }

    running_vcan.store(false, Ordering::SeqCst);

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
impl Into<SUnits> for UnitSystem {
    fn into(self) -> SUnits {
        match self {
            UnitSystem::SI => SUnits::SI,
            UnitSystem::USCS => SUnits::USCS,
        }
    }
}

impl From<SUnits> for UnitSystem {
    fn from(units: SUnits) -> Self {
        match units {
            SUnits::SI => UnitSystem::SI,
            SUnits::USCS => UnitSystem::USCS,
        }
    }
}
