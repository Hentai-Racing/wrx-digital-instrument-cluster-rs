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
                        use futures::stream::StreamExt;
                        use gpio_cdev::{
                            AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags,
                        };

                        let chip = Chip::new("/dev/gpiochip0");

                        if let Ok(mut chip) = chip {
                            if let Ok(line) = chip.get_line(8) {}
                        }
                    });
                }

                {
                    let gpio_1 = gpio_1.clone();
                    tokio::spawn(async move {
                        use uinput::event::keyboard::Key;

                        let device = create_uinput_dev(format!(
                            "{}-apalis_imx8-keyboard",
                            env!("CARGO_PKG_NAME")
                        ))
                        .ok();

                        if let Some(mut device) = device {
                            let mut gpio_1_watch = gpio_1.watch();
                            loop {
                                use tokio::select;

                                select! {
                                    Ok(_) = gpio_1_watch.changed() => {
                                        let value = *gpio_1_watch.borrow_and_update();

                                        if value {
                                            let _ = device.press(&Key::Up);
                                        } else {
                                            let _ = device.release(&Key::Up);
                                        }
                                    },
                                    else => {break;}
                                }
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
