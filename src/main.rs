mod application;
mod can;
mod data;
mod hardware;
mod slint_ui;

use crate::application::settings::SETTINGS;
use crate::can::can_backend::{CanBackend, CanFrame, CanInterface};
use crate::can::can_mux_parser::{
    self, ISOTPAckFrame, MuxContext, MuxParseResult, OBDService, S1CurrentData,
};
use crate::can::messages::emulators::wrx_2018_emulator;
use crate::can::messages::wrx_2018::CanError;
use crate::data::car_data::{CarData, ParseError, ParseResult};
use crate::hardware::hardware_backend::{self, HARDWARE_NAVIGATION_INPUT, HardwareBackend};
use crate::slint_ui::backend::{
    backend_lib, can_display::CanFrameDisplay, car_data_bridge, hardware_bridge, lang,
    rs_type_resolver, settings_bridge,
};

use clap::ArgMatches;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, System};
use tokio::sync::Notify;
use tokio::time::{self, Duration, Instant};

use std::collections::VecDeque;
use std::env;
use std::io::Write;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, LazyLock};

slint::include_modules!();

const VCAN_IF_NAME: &str = "vcan0";
const CAN_IF_NAME: &str = "can0";

#[cfg(target_os = "linux")]
const DEFAULT_SL_DEV: &str = "/dev/ttyACM0";
#[cfg(target_vendor = "apple")]
const DEFAULT_SL_DEV: &str = "/dev/tty.usbmodem101";

pub static SHUTDOWN_SIGNAL: LazyLock<Notify> = LazyLock::new(|| Notify::new());
pub static SHUTDOWN_FINISHED: LazyLock<Notify> = LazyLock::new(|| Notify::new());
pub static CAR_DATA: LazyLock<Arc<CarData>> = LazyLock::new(|| Default::default());
pub static BIN_ARGS: LazyLock<ArgMatches> = LazyLock::new(|| {
    let conflicting_args = vec![
        "fakedev",
        "candev",
        "sldev",
        #[cfg(target_os = "linux")]
        "virtual",
    ];

    clap::Command::new("").version(env!("CARGO_PKG_VERSION"))
        .args([
            #[cfg(target_os = "linux")]
            clap::arg!(-v --virtual "Runs the application in virtual mode using socketcan vcan")
                .required(false)
                .conflicts_with_all(conflicting_args.iter().filter(|&x| x != &"virtual")),
            clap::arg!(-f --fakedev "Runs the application in virtual mode using a fake can socket emulator")
                .required(false)
                .conflicts_with_all(conflicting_args.iter().filter(|&x| x != &"fakedev")),
            clap::arg!(-c --candev <PATH> "Path to the desired CAN device to use")
                .required(false)
                .default_value(CAN_IF_NAME)
                .conflicts_with_all(conflicting_args.iter().filter(|&x| x != &"candev")),
            clap::arg!(-s --sldev <PATH> "Path to the desired serial CAN device to use")
                .required(false)
                .default_value(DEFAULT_SL_DEV)
                .conflicts_with_all(conflicting_args.iter().filter(|&x| x != &"sldev")),
            #[cfg(debug_assertions)]
            clap::arg!(--cli "Enable internal cli")
                .required(false),
            #[cfg(debug_assertions)]
            clap::arg!(--highlight "Enables UI highlights")
                .required(false),
            #[cfg(debug_assertions)]
            clap::arg!(--no_overlay "Disables debug overlay")
                .required(false),
            ])
        .get_matches()
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fake_dev = BIN_ARGS.get_flag("fakedev");
    #[cfg(target_os = "linux")]
    let virtual_cluster = BIN_ARGS.get_flag("virtual") || fake_dev;
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
    } else if BIN_ARGS.value_source("candev") == Some(clap::parser::ValueSource::CommandLine) {
        CanInterface::SocketCan
    } else if BIN_ARGS.value_source("sldev") == Some(clap::parser::ValueSource::CommandLine) {
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

    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = tokio_runtime.enter();

    let ui = App::new()?;
    ui.show()?;

    let mut _created_interface = false;

    let mut interface_path = match &selected_interface {
        CanInterface::VirtualSocketCan | CanInterface::Fake => VCAN_IF_NAME,
        CanInterface::SerialCan => {
            if let Some(sldev) = BIN_ARGS.get_one::<String>("sldev") {
                sldev.as_str()
            } else {
                DEFAULT_SL_DEV
            }
        }
        CanInterface::SocketCan => {
            if let Some(candev) = BIN_ARGS.get_one::<String>("candev") {
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
        use can_mux_parser::S9VehicleInformation;

        let mut frame_display = CanFrameDisplay::new(ui.as_weak());

        tokio::spawn(async move {
            let obd_id =
                unsafe { embedded_can::Id::from(embedded_can::StandardId::new_unchecked(0x7E0)) };

            // TESTING
            let mut context = MuxContext::default();
            let mut queue = VecDeque::from(vec![
                // CanFrame::new(
                //     obd_id,
                //     8,
                //     &[
                //         0x02,
                //         OBDService::CurrentData.into(),
                //         S1CurrentData::PIDs1.into(),
                //     ],
                // ),
                // CanFrame::new(
                //     obd_id,
                //     8,
                //     &[
                //         0x02,
                //         OBDService::VehicleInformation.into(),
                //         S9VehicleInformation::PIDs.into(),
                //     ],
                // ),
                // CanFrame::new(
                //     obd_id,
                //     8,
                //     &[
                //         0x02,
                //         OBDService::VehicleInformation.into(),
                //         S9VehicleInformation::VIN.into(),
                //     ],
                // ),
                // CanFrame::new(obd_id, 8, &[0x01, OBDService::StoredDTCs.into()]),
            ]);

            let running_can = &SETTINGS.developer.can.running_can;

            loop {
                if running_can.value() {
                    if let Some(frame) = can_backend.read_frame() {
                        frame_display.update(&frame, false);

                        match CAR_DATA.parse_frame(&frame) {
                            Ok(result) => match result {
                                ParseResult::Mux(result) => match result {
                                    MuxParseResult::AwaitingBroadcastAck => {
                                        let ack = ISOTPAckFrame::new(obd_id);
                                        queue.push_front(CanFrame::from_frame(ack));
                                    }
                                    MuxParseResult::ConsecutiveFrameContinue => {
                                        context.waiting_for_responce = true;
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            Err(e) => match e {
                                ParseError::CanError(e) => match e {
                                    CanError::UnknownMessageId(_id) => {
                                        match context.parse_frame(&frame) {
                                            Ok(result) => match result {
                                                MuxParseResult::AwaitingBroadcastAck => {
                                                    let ack = ISOTPAckFrame::new(obd_id);
                                                    queue.push_front(CanFrame::from_frame(ack));
                                                }
                                                MuxParseResult::ConsecutiveFrameContinue => {
                                                    context.waiting_for_responce = true;
                                                }
                                                _ => {}
                                            },
                                            Err(e) => match e {
                                                can_mux_parser::MuxParseError::UnknownMessageId => {
                                                }
                                                _ => println!(
                                                    "Context failed to parse frame {frame:?}: {e:?}"
                                                ),
                                            },
                                        }
                                    }
                                    _ => println!("Failed to parse frame {frame:?}: {e:?}"),
                                },
                                _ => println!("Failed to parse frame {frame:?}: {e:?}"),
                            },
                        }
                    };

                    if !context.waiting_for_responce {
                        if let Some(frame) = queue.pop_front() {
                            match can_backend.write_frame(frame) {
                                Ok(_) => {
                                    context.waiting_for_responce = true;
                                    println!("Wrote frame: {frame:?}")
                                }
                                Err(e) => {
                                    eprintln!("Failed to write to can_socket: {e:?}");
                                }
                            }
                        }
                    }
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
                let mut interval = time::interval(Duration::from_millis(10));

                let running_vcan = &SETTINGS.developer.simulation.running_simulation;

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
    application_state.set_interface_type(format!("{selected_interface}: {interface_path}").into());

    #[cfg(feature = "apalis_imx8")]
    let device = hardware::apalis_imx8::ApalisIMX8::new();
    #[cfg(feature = "apalis_imx8")]
    let hardware_backend = Arc::new(HardwareBackend::new(hardware_backend::Backend::ApalisIMX8(
        device,
    )));
    #[cfg(not(feature = "apalis_imx8"))]
    let hardware_backend = Arc::new(HardwareBackend::new(hardware_backend::Backend::Simulator));

    tokio::spawn(run_autosave_loop());
    tokio::spawn(bridge_system_info());

    // ui backend bridges
    {
        rs_type_resolver::bridge(ui.as_weak());
        settings_bridge::bridge(ui.as_weak());
        car_data_bridge::bridge(ui.as_weak(), CAR_DATA.clone());
        hardware_bridge::bridge(ui.as_weak(), hardware_backend.clone());
        lang::bridge(ui.as_weak());
        backend_lib::bridge(ui.as_weak());
    }

    let frames: Arc<AtomicU32> = Default::default();

    {
        let frames = frames.clone();
        let _ = ui.window().set_rendering_notifier(move |state, _api| {
            if matches!(state, slint::RenderingState::BeforeRendering) {
                frames.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
    }
    {
        let frames = frames.clone();
        tokio::spawn(async move {
            let mut last = Instant::now();
            let mut interval = time::interval(Duration::from_millis(100));

            loop {
                interval.tick().await;
                let secs = last.elapsed().as_secs_f32();

                SETTINGS.developer.system_info.fps.set_value(
                    ((frames.swap(0, std::sync::atomic::Ordering::Relaxed) as f32) / secs) as i32,
                );

                last = Instant::now();
            }
        });
    }

    // graceful shutdown
    {
        use tokio::signal;

        tokio::spawn(async {
            match signal::ctrl_c().await {
                Ok(_) => SHUTDOWN_SIGNAL.notify_one(),
                Err(e) => panic!("Failed to forward shutdown signal: {e:?}"),
            }
        });

        tokio::spawn(async {
            SHUTDOWN_SIGNAL.notified().await;
            shutdown();
        });
    }

    init_state();
    ui.run()?;

    SHUTDOWN_SIGNAL.notify_one();

    tokio_runtime.block_on(async { SHUTDOWN_FINISHED.notified().await });
    tokio_runtime.shutdown_background();

    Ok(())
}

fn save_config() {
    if let Err(e) = SETTINGS.save_to_fs() {
        eprintln!("Failed to save user settings: {e:?}");
    }
}

fn init_state() {
    #[cfg(debug_assertions)]
    {
        if BIN_ARGS.get_flag("highlight") {
            SETTINGS.developer.debug.debug_highlights.set_value(true);
        }

        if BIN_ARGS.get_flag("no_overlay") {
            SETTINGS
                .developer
                .debug
                .debug_overlay_enabled
                .set_value(false);
        }

        if BIN_ARGS.get_flag("cli") {
            tokio::spawn(cli_mode());
        }
    }
}

fn shutdown() {
    if let Err(e) = slint::quit_event_loop() {
        panic!("Failed to quit Slint event loop: {e:?}");
    }

    save_config();

    SHUTDOWN_FINISHED.notify_one();
}

async fn run_autosave_loop() {
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        save_config();
    }
}

#[cfg(debug_assertions)]
async fn cli_mode() {
    use std::io::{stdin, stdout};

    let mut buf = String::new();
    let cli_in = stdin();
    let mut cli_out = stdout();

    loop {
        print!("CLI > ");
        let _ = cli_out.flush();

        if cli_in.read_line(&mut buf).is_ok() {
            if buf.ends_with('\n') {
                buf.pop();
                if buf.ends_with('\r') {
                    buf.pop();
                }
            }

            let command = buf.to_lowercase();
            let cmd_splt: Vec<&str> = command.as_str().split(" ").collect();
            match *cmd_splt.get(0).unwrap_or(&"") {
                "q" | "quit" => {
                    SHUTDOWN_SIGNAL.notify_one();
                    break;
                }
                "h" | "help" => {
                    println!();
                    println!("h | help     => show this help menu");

                    println!("\nq | quit     => close application");

                    println!("\nnav up       => force ui navigation up");
                    println!("    down     => force ui navigation down");
                    println!("    enter    => force ui navigation enter");

                    println!(
                        "\nset_param    => set the value of any `<Parameter>` in `{}`",
                        stringify!(CONFIG_MANAGER)
                    );
                    println!("    usage    |  set_param <path> <value>");
                    println!("  example    |  set_param user.general.unit_system uscs");

                    println!(
                        "\nparam_layout => show the layout of `{}`",
                        stringify!(CONFIG_MANAGER)
                    );

                    println!("\nset_car_data => set the value of any `<Parameter>` in `CAR_DATA`");
                    println!("    usage    |  set_car_data <param> <value>");
                    println!("  example    |  set_car_data engine_rpm 1234");
                    println!("     note    |  does not update immediately due to unknown bug");
                    println!();
                }
                "nav" => {
                    match *cmd_splt.get(1).unwrap_or(&"") {
                        "up" => {
                            HARDWARE_NAVIGATION_INPUT
                                .set_value(hardware_backend::HardwareNavigationState::Backward);
                        }
                        "down" => {
                            HARDWARE_NAVIGATION_INPUT
                                .set_value(hardware_backend::HardwareNavigationState::Forward);
                        }
                        "enter" => {
                            HARDWARE_NAVIGATION_INPUT
                                .set_value(hardware_backend::HardwareNavigationState::Enter);
                        }
                        _ => {}
                    };
                    let mut interval = time::interval(Duration::from_millis(100));
                    interval.tick().await;
                    interval.tick().await;
                    println!("resetting input");
                    HARDWARE_NAVIGATION_INPUT
                        .set_value(hardware_backend::HardwareNavigationState::Idle);
                }
                "set_param" => {
                    let param_path = *cmd_splt.get(1).unwrap_or(&"");
                    let value = *cmd_splt.get(2).unwrap_or(&"");
                    SETTINGS.set_by_path(param_path, value);
                }
                "param_layout" => {
                    println!("\n{}", SETTINGS.get_page_layout());
                }
                "set_car_data" => {
                    // FIXME: if canbus and sim are both disabled, you must update multiple cardata params before the ui updates
                    let param = *cmd_splt.get(1).unwrap_or(&"");
                    let value = *cmd_splt.get(2).unwrap_or(&"");

                    CAR_DATA.set_param_by_name(param, value);
                }
                x => eprintln!("Unknown command `{x}`"),
            }
            buf.clear();
        }
    }
}

async fn bridge_system_info() {
    let pid = Pid::from_u32(std::process::id());

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut process_memory_max = 0;

    let sys_refresh = RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::nothing().with_ram())
        .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
        .with_processes(ProcessRefreshKind::nothing().with_memory());

    SETTINGS
        .developer
        .system_info
        .num_cpus
        .set_value(sys.cpus().len() as i32);

    SETTINGS
        .developer
        .system_info
        .total_memory_mb
        .set_value((sys.total_memory() / 1_048_576) as i32);

    let mut interval = time::interval(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    loop {
        sys.refresh_specifics(sys_refresh);
        sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), false);

        let used_memory = (sys.used_memory() / 1_048_576) as i32;

        if let Some(this) = sys.process(pid) {
            let process_memory = (this.memory() / 1_048_576) as i32;
            process_memory_max = std::cmp::max(process_memory, process_memory_max);

            SETTINGS
                .developer
                .system_info
                .process_memory_mb
                .set_value(process_memory);

            SETTINGS
                .developer
                .system_info
                .process_memory_max_mb
                .set_value(process_memory_max);
        }

        SETTINGS
            .developer
            .system_info
            .used_memory_mb
            .set_value(used_memory);

        SETTINGS
            .developer
            .system_info
            .cpu_usage
            .set_value(sys.global_cpu_usage());

        interval.tick().await;
    }
}
