mod application;
mod can;
mod data;
mod hardware;
mod slint_ui;

use crate::application::settings::{SaveStatus, SettingsManager};
use crate::can::can_backend::{CanBackend, CanFrame, CanInterface};
use crate::can::can_mux_manager::{ISOTPAckFrame, MuxParseResult, OBD2Service};
use crate::can::messages::emulators::wrx_2018_emulator;
use crate::can::messages::wrx_2018::CanError;
use crate::data::car_data::{CarData, ParseError, ParseResult};
use crate::hardware::hardware_backend::{self, HardwareBackend};
use crate::slint_ui::backend::{
    can_display::CanFrameDisplay, car_data_bridge, hardware_bridge, user_settings_bridge,
};

use tokio::sync::mpsc;

use std::collections::VecDeque;
use std::env;
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

slint::include_modules!();

const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";

#[cfg(target_os = "linux")]
const DEFAULT_SL_DEV: &str = "/dev/ttyACM0";
#[cfg(target_vendor = "apple")]
const DEFAULT_SL_DEV: &str = "/dev/tty.usbmodem101";

static SETTINGS_MANAGER: LazyLock<Arc<SettingsManager>> = LazyLock::new(|| Default::default());
static CAR_DATA: LazyLock<Arc<CarData>> = LazyLock::new(|| Default::default());

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = clap::Command::new("").version(env!("CARGO_PKG_VERSION"))
        .args([
            #[cfg(target_os = "linux")]
            clap::arg!(-v --virtual "Runs the application in virtual mode using socketcan vcan")
                .required(false)
                .exclusive(true),
            clap::arg!(-f --fakedev "Runs the application in virtual mode using a fake can socket emulator")
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

    let fake_dev = cli.get_flag("fakedev");

    #[cfg(target_os = "linux")]
    let virtual_cluster = cli.get_flag("virtual") | fake_dev;
    #[cfg(not(target_os = "linux"))]
    let virtual_cluster = fake_dev;

    let selected_interface = if fake_dev {
        CanInterface::Fake
    } else if virtual_cluster {
        #[cfg(target_os = "linux")]
        {
            CanInterface::VirtualSocketCan
        }

        #[cfg(not(target_os = "linux"))]
        {
            CanInterface::Fake
        }
    } else if cli.value_source("candev") == Some(clap::parser::ValueSource::CommandLine) {
        CanInterface::SocketCan
    } else if cli.value_source("sldev") == Some(clap::parser::ValueSource::CommandLine) {
        CanInterface::SerialCan
    } else {
        #[cfg(feature = "apalis_imx8")]
        {
            CanInterface::SocketCan
        }
        #[cfg(not(feature = "apalis_imx8"))]
        {
            CanInterface::SerialCan
        }
    };

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = tokio_runtime.enter();

    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<bool>();
    let (shutdown_finished, mut shutdown_finished_recv) = mpsc::unbounded_channel::<bool>();

    #[cfg(debug_assertions)]
    if env::var("SLINT_DEBUG_PERFORMANCE")
        .unwrap_or_default()
        .is_empty()
    {
        unsafe {
            env::set_var("SLINT_DEBUG_PERFORMANCE", "refresh_full_speed,overlay");
        }
    }

    let ui = App::new()?;
    ui.show()?;

    let mut _created_interface = false;

    let mut interface_path = match &selected_interface {
        CanInterface::VirtualSocketCan | CanInterface::Fake => VCAN_IF_NAME,
        CanInterface::SerialCan => {
            if let Some(sldev) = cli.get_one::<String>("sldev") {
                sldev.as_str()
            } else {
                DEFAULT_SL_DEV
            }
        }
        CanInterface::SocketCan => {
            if let Some(candev) = cli.get_one::<String>("candev") {
                candev.as_str()
            } else {
                CAN_IF_NAME
            }
        }
    };

    let can_backend = match CanBackend::new(&selected_interface, interface_path) {
        Ok(can_backend) => Some(can_backend),
        Err(e) => {
            eprintln!("Error in can backend: {e:?}");
            interface_path = "err";
            None
        }
    };

    if let Some(mut can_backend) = can_backend {
        let mut frame_display = CanFrameDisplay::new(ui.as_weak());
        let car_data = CAR_DATA.clone();

        tokio::spawn(async move {
            let obd_id =
                unsafe { embedded_can::Id::from(embedded_can::StandardId::new_unchecked(0x7E0)) };

            // TESTING
            let mut queue = VecDeque::from(vec![
                CanFrame::new(obd_id, 8, &[0x02, OBD2Service::CurrentData.into(), 0x0c]),
                CanFrame::new(
                    obd_id,
                    8,
                    &[0x02, OBD2Service::VehicleInformation.into(), 0x00],
                ),
                CanFrame::new(
                    obd_id,
                    8,
                    &[0x02, OBD2Service::VehicleInformation.into(), 0x02],
                ),
            ]);

            let running_can = &SETTINGS_MANAGER.session_settings.can_settings.running_can;

            loop {
                if running_can.value() {
                    if let Some(frame) = can_backend.read_frame() {
                        frame_display.update(&frame, false);
                        match car_data.parse_frame(&frame) {
                            Ok(result) => match result {
                                ParseResult::Mux(result) => match result {
                                    MuxParseResult::AwaitingBroadcastAck => {
                                        let ack = ISOTPAckFrame::new(obd_id);
                                        queue.push_front(CanFrame::from_frame(ack));
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            Err(e) => match e {
                                ParseError::CanError(e) => match e {
                                    CanError::UnknownMessageId(_id) => {
                                        // ignore
                                    }
                                    _ => println!("Failed to parse frame: {e:?}"),
                                },
                                _ => println!("Failed to parse frame: {e:?}"),
                            },
                        }
                    };

                    // if !car_data.obd_mux_context.waiting_for_responce {
                    //     if let Some(frame) = queue.pop_front() {
                    //         match can_backend.write_frame(frame) {
                    //             Ok(_written_bytes) => {
                    //                 car_data.obd_mux_context.waiting_for_responce = true;
                    //             }
                    //             Err(e) => {
                    //                 eprintln!("Failed to write to can_socket: {e:?}");
                    //             }
                    //         }
                    //     }
                    // }
                } else {
                    let _ = running_can.watch().wait_for(|v| *v).await;
                }
            }
        });

        if virtual_cluster {
            let mut backend = CanBackend::new(&selected_interface, interface_path).unwrap();

            if !matches!(
                &selected_interface,
                CanInterface::Fake | CanInterface::VirtualSocketCan
            ) {
                panic!("Do not run the can generator on a real socket!")
            }

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(10));

                let running_vcan = &SETTINGS_MANAGER
                    .session_settings
                    .simulation_settings
                    .simulation_running;

                loop {
                    let gen_frames = wrx_2018_emulator::generate_frames();

                    for frame in gen_frames {
                        if running_vcan.value() {
                            if let Err(e) = backend.write_frame(frame) {
                                eprintln!("Failed to write simulated frame: {e:?}");
                            }
                        } else {
                            let _ = running_vcan.watch().wait_for(|v| *v).await;
                        }
                    }

                    interval.tick().await;
                }
            });
        }
    }

    let application_state = ui.global::<ApplicationState>();

    application_state.set_virtual_cluster(virtual_cluster);
    application_state.set_interface_type(
        format!(
            "{}: {}",
            match selected_interface {
                CanInterface::Fake => "fake",
                CanInterface::SerialCan => "slcan",
                CanInterface::SocketCan => "socketcan",
                CanInterface::VirtualSocketCan => "vcan(socketcan)",
            },
            interface_path
        )
        .into(),
    );
    application_state.set_debug_mode(cfg!(debug_assertions));

    SETTINGS_MANAGER.load_from_fs()?;

    #[cfg(feature = "apalis_imx8")]
    let device = hardware::apalis_imx8::ApalisIMX8::new();
    #[cfg(feature = "apalis_imx8")]
    let hardware_backend = Arc::new(HardwareBackend::new(hardware_backend::Backend::ApalisIMX8(
        device,
    )));
    #[cfg(not(feature = "apalis_imx8"))]
    let hardware_backend = Arc::new(HardwareBackend::new(hardware_backend::Backend::Simulator));

    user_settings_bridge::bridge(ui.as_weak(), SETTINGS_MANAGER.clone());
    car_data_bridge::bridge(ui.as_weak(), CAR_DATA.clone(), SETTINGS_MANAGER.clone());
    hardware_bridge::bridge(ui.as_weak(), hardware_backend.clone());

    // main loop

    // autosave interval
    tokio::spawn(async move {
        let mut now = Instant::now();
        loop {
            if now.elapsed() >= Duration::from_secs(30) {
                now = Instant::now();
                save_settings();
            }
        }
    });

    let cleanup = move || {
        if let Err(e) = slint::quit_event_loop() {
            panic!("Failed to quit Slint event loop: {e:?}");
        }

        save_settings();

        if let Err(e) = shutdown_finished.send(true) {
            panic!("Failed to send `shutdown_finished` signal: {e:?}")
        }
    };

    // graceful shutdown
    {
        use tokio::signal;

        let shutdown_send = shutdown_send.clone();
        tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(_) => shutdown_send.send(true),
                Err(e) => panic!("Failed to forward shutdown signal: {e:?}"),
            }
        });

        tokio::spawn(async move {
            match shutdown_recv.recv().await {
                Some(_) => {
                    cleanup();
                }
                _ => {}
            }
        });
    }

    ui.run()?;

    if !shutdown_send.is_closed() {
        if let Err(e) = shutdown_send.send(true) {
            panic!("Failed to send shutdown signal: {e:?}");
        }
    }

    tokio_runtime.block_on(async move { shutdown_finished_recv.recv().await });
    tokio_runtime.shutdown_background();

    Ok(())
}

fn save_settings() {
    //! TODO: possible failure when saving where the file will be incomplete, causing reloading to be paused indefinitely
    match SETTINGS_MANAGER.save_to_fs() {
        Ok(status) => match status {
            SaveStatus::Success => {}
            SaveStatus::Failed(e) => eprintln!("Failed to write settings: {e:?}"),
        },
        Err(e) => eprintln!("Failed to save settings: {e:?}"),
    }
}
