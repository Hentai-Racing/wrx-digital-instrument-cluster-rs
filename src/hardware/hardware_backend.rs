use crate::data::parameters::FieldParameter;
#[cfg(feature = "apalis_imx8")]
use crate::hardware::apalis_imx8;

pub enum Backend {
    #[cfg(feature = "apalis_imx8")]
    ApalisIMX8(apalis_imx8::ApalisIMX8),
    None,
}

pub struct HardwareBackend {
    backend: Backend,
}

#[cfg(feature = "apalis_imx8")]
fn create_uinput_dev(name: String) -> Result<uinput::Device, Box<dyn std::error::Error>> {
    use uinput::event::keyboard::Key;

    let dev = uinput::default()?
        .name(name)?
        .event(Key::Up)?
        .event(Key::Down)?
        .event(Key::Enter)?
        .create()?;

    Ok(dev)
}

impl HardwareBackend {
    pub fn new(backend: Backend) -> Self {
        match &backend {
            #[cfg(feature = "apalis_imx8")]
            Backend::ApalisIMX8(_) => {
                let gpio_1 = FieldParameter::<bool>::new(false);

                {
                    let mut gpio_1 = gpio_1.clone();
                    tokio::spawn(async move {
                        use gpio_cdev::{Chip, EventRequestFlags, EventType, LineRequestFlags};
                        use std::os::unix::io::AsRawFd;
                        use tokio::io::unix::AsyncFd;

                        let mut chip = match Chip::new("/dev/gpiochip0") {
                            Ok(chip) => chip,
                            Err(e) => {
                                eprintln!("chip error: {e}");
                                return;
                            }
                        };

                        let line = match chip.get_line(8) {
                            Ok(line) => line,
                            Err(e) => {
                                eprintln!("line error: {e}");
                                return;
                            }
                        };

                        let mut event_handle = match line.events(
                            LineRequestFlags::INPUT,
                            EventRequestFlags::BOTH_EDGES,
                            "gpio-async",
                        ) {
                            Ok(event_handle) => event_handle,
                            Err(e) => {
                                eprintln!("event request error: {e}");
                                return;
                            }
                        };

                        let async_fd = match AsyncFd::new(event_handle.as_raw_fd()) {
                            Ok(fd) => fd,
                            Err(e) => {
                                eprintln!("async fd error: {e}");
                                return;
                            }
                        };

                        loop {
                            match async_fd.readable().await {
                                Ok(mut guard) => {
                                    match event_handle.get_event() {
                                        Ok(event) => match event.event_type() {
                                            EventType::RisingEdge => {
                                                gpio_1.set_value(true);
                                            }
                                            EventType::FallingEdge => {
                                                gpio_1.set_value(false);
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("event read error: {e}");
                                        }
                                    }

                                    guard.clear_ready();
                                }
                                Err(e) => {
                                    eprintln!("await error: {e}");
                                    break;
                                }
                            }
                        }
                    });
                }

                {
                    let gpio_1 = gpio_1.clone();
                    tokio::spawn(async move {
                        use uinput::event::keyboard::Key;

                        let mut device = match create_uinput_dev(format!(
                            "{}-apalis_imx8-keyboard",
                            env!("CARGO_PKG_NAME")
                        )) {
                            Ok(device) => device,
                            Err(e) => {
                                eprintln!("event request error: {e}");
                                return;
                            }
                        };

                        let mut gpio_1_watch = gpio_1.watch();
                        loop {
                            use tokio::select;

                            select! {
                                Ok(_) = gpio_1_watch.changed() => {
                                    let value = *gpio_1_watch.borrow_and_update();

                                    if value {
                                        let _ = device.press(&Key::Up);
                                        println!("fake keyboard press");
                                    } else {
                                        let _ = device.release(&Key::Up);
                                        println!("fake keyboard release");
                                    }
                                },
                                else => {break;}
                            }

                            if let Err(e) = device.synchronize() {
                                eprintln!("Failed to synchronize uinput device: {e}");
                            }
                        }
                    });
                }
            }
            _ => {}
        }

        Self { backend }
    }
}
