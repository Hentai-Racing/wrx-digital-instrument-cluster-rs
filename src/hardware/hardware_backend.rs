use crate::data::parameters::FieldParameter;
#[cfg(feature = "apalis_imx8")]
use crate::hardware::apalis_imx8;
use tokio::spawn;

pub enum Backend {
    #[cfg(feature = "apalis_imx8")]
    ApalisIMX8(apalis_imx8::ApalisIMX8),
    None,
}

pub struct HardwareBackend {
    backend: Backend,
}

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
                spawn(async {
                    use futures::stream::StreamExt;
                    use gpio_cdev::{
                        AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags,
                    };
                    use uinput::event::keyboard::Key;

                    let device = create_uinput_dev(format!(
                        "{}-apalis_imx8-keyboard",
                        env!("CARGO_PKG_NAME")
                    ))
                    .ok();

                    let chip = Chip::new("/dev/gpiochip0");

                    if let Some(mut device) = device {
                        if let Ok(mut chip) = chip {
                            if let Ok(line) = chip.get_line(8) {
                                let mut events = AsyncLineEventHandle::new(
                                    line.events(
                                        LineRequestFlags::INPUT,
                                        EventRequestFlags::BOTH_EDGES,
                                        "gpioevents",
                                    )
                                    .unwrap(),
                                )
                                .unwrap();

                                while let Some(event) = events.next().await {
                                    if let Ok(event) = event {
                                        match event.event_type() {
                                            gpio_cdev::EventType::FallingEdge => {
                                                let _ = device.press(&Key::Up);
                                            }
                                            gpio_cdev::EventType::RisingEdge => {
                                                let _ = device.release(&Key::Down);
                                            }
                                        }

                                        println!("GPIO Event: {:?}", event);
                                    }
                                }
                            }
                        }
                    }
                });
            }
            _ => {}
        }

        Self { backend }
    }
}
