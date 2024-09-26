mod can;
mod car_data;
mod unit_conversion;
use crate::can::messages::wrx_2018;
use crate::can::virtual_can_generator::run_vcan_generator;
use car_data::CarData;
use futures::stream::StreamExt;
use slint::{ComponentHandle, Weak};
use socketcan::tokio::CanSocket;
use socketcan::CanInterface;
use std::env;
use std::string::ToString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::{signal, task};
use unit_conversion::Units;

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

    let running_vcan = Arc::new(AtomicBool::new(false));
    let mut vcan_task: Option<task::JoinHandle<()>> = None;

    let mut car_data: Arc<CarData> = Arc::new(CarData { engine_rpm: 0 });

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
            let mut can_socket = CanSocket::open(can_if_name).expect("Failed to open can socket");

            task::spawn(async move {
                read_can_frames(&mut can_socket).await;
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

    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();

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

async fn read_can_frames(can_socket: &mut socketcan::tokio::CanSocket) {
    while let Some(Ok(frame)) = can_socket.next().await {
        parse_can_frame(frame);
    }
}

fn parse_can_frame(frame: impl embedded_can::Frame) {
    use wrx_2018::Messages;

    match Messages::from_can_message(frame.id(), frame.data()) {
        Ok(message) => match message {
            _ => {
                todo!()
            }
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
