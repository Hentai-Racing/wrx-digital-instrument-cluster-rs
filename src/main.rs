mod can;
mod can_data_bridge;
mod data;

use can::messages::wrx_2018;
use can::virtual_can_generator::run_vcan_generator;
use can_data_bridge::CanDataBridge;
use data::car_data::CarData;
use data::units;
use socketcan::tokio::CanSocket;
use socketcan::CanInterface;
use std::env;
use std::string::ToString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::{signal, task};
use units::UnitSystem;

slint::include_modules!();

const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";

#[tokio::main]
async fn main() {
    let virtual_cluster = env::var("HR_CLUSTER_VIRTUAL").is_ok_and(|val| val == "1");
    let mut created_vcan = false;

    let in_use_can_if_name: Option<&str>;
    if virtual_cluster {
        in_use_can_if_name = match CanInterface::open(&VCAN_IF_NAME) {
            Ok(_) => Some(VCAN_IF_NAME),
            Err(_) => match CanInterface::create_vcan(&VCAN_IF_NAME, None) {
                Ok(_) => Some(VCAN_IF_NAME).inspect(|vcan_if_name| {
                    created_vcan = true;
                    println!("Created virtual CAN interface {vcan_if_name}")
                }),
                Err(e) => {
                    eprintln!("Failed to create virtual CAN interface {VCAN_IF_NAME}: {e}");
                    None
                }
            },
        }
    } else {
        in_use_can_if_name = match CanInterface::open(&CAN_IF_NAME) {
            Ok(_) => Some(CAN_IF_NAME),
            Err(e) => {
                eprintln!("Failed to open CAN interface {CAN_IF_NAME}: {e}");
                None
            }
        }
    }

    let running_vcan = Arc::new(AtomicBool::new(false)); // todo: is this necessary?
    let mut vcan_task: Option<task::JoinHandle<()>> = None;
    let init_ui = false;

    let car_data = CarData::new();

    if let Some(can_if_name) = in_use_can_if_name {
        println!("Using CAN interface {can_if_name}");

        let socket_up =
            CanInterface::open(can_if_name).is_ok_and(|interface| match interface.details() {
                Ok(details) => {
                    if details.is_up {
                        true
                    } else {
                        match interface.bring_up() {
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
            });

        if socket_up {
            let can_socket = CanSocket::open(can_if_name).expect("Failed to open can socket");
            let mut can_data_bridge = CanDataBridge::new(car_data, can_socket);

            task::spawn(async move {
                can_data_bridge.read_can_frames().await;
            });

            if virtual_cluster {
                let mut virtual_socket =
                    CanSocket::open(can_if_name).expect("Failed to open can socket");

                let task_arc = running_vcan.clone();
                task_arc.store(true, Ordering::SeqCst);

                vcan_task = Some(task::spawn(async move {
                    run_vcan_generator(&mut virtual_socket, task_arc).await
                }));
            }
        }
    }

    if init_ui {
        let ui = AppWindow::new().unwrap();
        ui.run().unwrap();
    } else {
        println!("Ctrl+C to stop");
        signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl_c signal");
        println!();
    }

    if let Some(vcan_task) = vcan_task {
        running_vcan.store(false, Ordering::SeqCst); // stop the task loop

        vcan_task.abort(); // wait for the task to be aborted before deleting the interface
    }

    if created_vcan {
        match CanInterface::open(VCAN_IF_NAME) {
            Ok(vcan_interface) => match vcan_interface.delete() {
                Ok(_) => println!("Deleted interface {VCAN_IF_NAME}"),
                Err(e) => println!("Error deleting interface {VCAN_IF_NAME}: {e:?}"),
            },
            Err(e) => println!("Error opening interface when deleting {VCAN_IF_NAME}: {e}"),
        }
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
