mod application;
mod can;
mod data;
mod hardware;
mod ui;

use application::settings::SettingsManager;

use crate::can::can_backend::{CanBackend, CanFrame, SelectedCanInterface};
use crate::can::can_data_emulator::run_can_data_emulator;
use crate::can::can_mux_manager::{ISOTPAckFrame, MuxParseResult, OBD2Service};
use crate::data::car_data::{CarData, ParseResult};
use crate::data::parameters::FieldParameter;
use crate::data::units::UnitSystem;
use crate::ui::car_data_bridge::SCarDataBridge;
use crate::ui::{can_display::CanFrameDisplay, user_settings_bridge};

use tokio::select;
use tokio::sync::mpsc;

use std::collections::VecDeque;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

slint::include_modules!();

#[allow(unused)]
const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";

#[cfg(target_os = "linux")]
const DEFAULT_SL_DEV: &str = "/dev/ttyACM0";
#[cfg(target_vendor = "apple")]
const DEFAULT_SL_DEV: &str = "/dev/tty.usbmodem101";

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
        SelectedCanInterface::Fake
    } else if virtual_cluster {
        #[cfg(target_os = "linux")]
        {
            SelectedCanInterface::VirtualSocketCan
        }

        #[cfg(not(target_os = "linux"))]
        {
            SelectedCanInterface::Fake
        }
    } else if cli.value_source("candev") == Some(clap::parser::ValueSource::CommandLine) {
        SelectedCanInterface::SocketCan
    } else if cli.value_source("sldev") == Some(clap::parser::ValueSource::CommandLine) {
        SelectedCanInterface::SerialCan
    } else {
        #[cfg(feature = "apalis_imx8")]
        {
            SelectedCanInterface::SocketCan
        }
        #[cfg(not(feature = "apalis_imx8"))]
        {
            SelectedCanInterface::SerialCan
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

    let mut handles = vec![];
    let mut runners = vec![];

    let car_data = CarData::new();
    let settings_manager = Arc::new(RwLock::new(SettingsManager::default()));

    let running_simulation = Arc::new(AtomicBool::new(false));
    let mut _created_interface = false;

    let interface_path = match &selected_interface {
        SelectedCanInterface::VirtualSocketCan | SelectedCanInterface::Fake => VCAN_IF_NAME,
        SelectedCanInterface::SerialCan => {
            if let Some(sldev) = cli.get_one::<String>("sldev") {
                sldev.as_str()
            } else {
                DEFAULT_SL_DEV
            }
        }
        SelectedCanInterface::SocketCan => {
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
            None
        }
    };

    if let Some(mut can_backend) = can_backend {
        let mut frame_display = CanFrameDisplay::new(ui.as_weak());
        let mut car_data = car_data.clone();
        let settings_manager = settings_manager.clone();

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

            loop {
                if let Ok(settings_manager) = settings_manager.try_read() {
                    let running_can = settings_manager
                        .session_settings
                        .can_settings
                        .running_can
                        .value();

                    if running_can {
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
                                Err(e) => println!("Failed to parse frame: {e:?}"),
                            }
                        };

                        if !car_data.obd_mux_context.waiting_for_responce {
                            if let Some(frame) = queue.pop_front() {
                                match can_backend.write_frame(frame) {
                                    Ok(_written_bytes) => {
                                        car_data.obd_mux_context.waiting_for_responce = true;
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to write to can_socket: {e:?}");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        if virtual_cluster {
            let running_vcan = Arc::new(AtomicBool::new(false));

            running_simulation.store(true, Ordering::SeqCst);
            running_vcan.store(true, Ordering::SeqCst);

            let running_simulation_clone = running_simulation.clone();
            let running_vcan_clone = running_vcan.clone();
            let mut backend = CanBackend::new(&selected_interface, interface_path).unwrap();

            if !matches!(
                &selected_interface,
                SelectedCanInterface::Fake | SelectedCanInterface::VirtualSocketCan
            ) {
                panic!("Do not run the can generator on a real socket!")
            }

            let vcan_handle = thread::spawn(move || {
                // TODO: change generator function to only generate once, and move the loop logic here
                run_can_data_emulator(
                    &mut backend,
                    running_vcan_clone,
                    running_simulation_clone,
                    Duration::from_millis(1),
                );
            });

            handles.push(vcan_handle);
            runners.push(running_vcan);
        }
    }
    runners.push(running_simulation.clone());

    let debug_menu_state = ui.global::<DebugMenuState>();
    let application_state = ui.global::<ApplicationState>();

    application_state.set_virtual_cluster(virtual_cluster);
    application_state.set_debug_mode(cfg!(debug_assertions));

    if let Ok(mut settings_manager) = settings_manager.write() {
        settings_manager.load_from_fs()?;
    }

    let unit_system_parameter = match settings_manager.read() {
        Ok(settings_manager) => settings_manager.user_settings.general.unit_system.clone(),
        _ => FieldParameter::from(UnitSystem::default()),
    };

    user_settings_bridge::bridge_settings(ui.as_weak().clone(), settings_manager.clone());

    let mut ui_data_bridge =
        SCarDataBridge::new(ui.as_weak(), car_data.clone(), unit_system_parameter);
    ui_data_bridge.run();

    #[cfg(feature = "apalis_imx8")]
    {
        use crate::hardware::{apalis_imx8::ApalisIMX8, hardware_backend};

        let device = ApalisIMX8::new();

        let hardware_backend =
            hardware_backend::HardwareBackend::new(hardware_backend::Backend::ApalisIMX8(device));

        debug_menu_state.on_debug_suspend(move || {
            // device.power_suspend();
        });
    }
    #[cfg(not(feature = "apalis_imx8"))]
    {
        use crate::hardware::hardware_backend;

        let hardware_backend =
            hardware_backend::HardwareBackend::new(hardware_backend::Backend::None);
        debug_menu_state.on_debug_suspend(|| println!("DEBUG: DO SUSPEND"));
    }

    // main loop

    // temp cardata :-> settings_manager bind hack
    {
        let car_data = car_data.clone();
        let settings_manager = settings_manager.clone();
        tokio::spawn(async move {
            let mut watch = car_data.odometer().watch();

            loop {
                if let Ok(_) = watch.changed().await {
                    let val = *watch.borrow_and_update();

                    if let Ok(mut settings_manager) = settings_manager.try_write() {
                        settings_manager
                            .user_settings
                            .static_car_data
                            .odometer
                            .set_value(val as u32);
                    }
                }
            }
        });
    }

    let save_settings_settings_manager = settings_manager.clone();
    let save_settings = move || {
        if let Ok(settings_manager) = save_settings_settings_manager.read() {
            let _ = settings_manager.save_to_fs();
        }
    };

    // autosave interval
    {
        use std::time::{Duration, Instant};

        let save_settings = save_settings.clone();
        tokio::spawn(async move {
            let mut now = Instant::now();
            loop {
                if now.elapsed() >= Duration::from_secs(30) {
                    now = Instant::now();
                    save_settings();
                }
            }
        });
    }

    let cleanup = move || {
        for runner in runners {
            runner.store(false, Ordering::SeqCst);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        if let Err(e) = slint::quit_event_loop() {
            panic!("Failed to quit Slint event loop: {e:?}");
        }

        save_settings();
        if let Err(e) = shutdown_finished.send(true) {
            panic!("Failed to send `shutdown_finished` signal: {e:?}")
        }
    };

    // running_simulation bind
    {
        let settings_manager = settings_manager.clone();
        let running_simulation = running_simulation.clone();

        tokio::spawn(async move {
            let mut simulation_running_setting = None;
            if let Ok(settings_manager) = settings_manager.read() {
                simulation_running_setting = Some(
                    settings_manager
                        .session_settings
                        .simulation_settings
                        .simulation_running
                        .watch(),
                );
            }

            if let Some(mut simulation_running_setting) = simulation_running_setting {
                loop {
                    select! {
                        Ok(_) = simulation_running_setting.changed() => {
                            let value = *simulation_running_setting.borrow_and_update();
                            running_simulation.store(value, Ordering::SeqCst);
                        },
                        else => {
                            break;
                        },
                    }
                }
            }
        });
    }

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
