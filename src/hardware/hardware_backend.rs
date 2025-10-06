use crate::data::parameters::Parameter;
#[cfg(feature = "apalis_imx8")]
use crate::hardware::apalis_imx8;

use std::sync::Arc;

#[derive(Default)]
pub enum Backend {
    #[cfg(feature = "apalis_imx8")]
    ApalisIMX8(apalis_imx8::ApalisIMX8),
    #[default]
    Simulator,
}

#[derive(Default)]
pub struct HardwareBackend {
    backend: Backend,
    pub nav_forward: Arc<Parameter<bool>>,
    pub nav_backward: Arc<Parameter<bool>>,
    pub nav_enter: Arc<Parameter<bool>>,
}

impl HardwareBackend {
    pub fn new(backend: Backend) -> Self {
        match &backend {
            #[cfg(feature = "apalis_imx8")]
            Backend::ApalisIMX8(apalis_imx8) => {
                use crate::hardware::apalis_imx8::{ApalisIMX8, ApalisIMX8ADC, ApalisIMX8GPIO};

                let nav_forward: Arc<Parameter<bool>> = Default::default();
                let nav_backward: Arc<Parameter<bool>> = Default::default();
                let nav_enter: Arc<Parameter<bool>> = Default::default();

                apalis_imx8.register_gpio_reader(ApalisIMX8GPIO::GPIO1);
                let gpio_1 = apalis_imx8.get_gpio_param(ApalisIMX8GPIO::GPIO1);

                {
                    let gpio_1 = gpio_1.clone();
                    let nav_forward = nav_forward.clone();
                    tokio::spawn(async move {
                        let mut gpio_1_watch = gpio_1.watch();
                        loop {
                            use tokio::select;

                            select! {
                                Ok(_) = gpio_1_watch.changed() => {
                                    nav_forward.set_value(*gpio_1_watch.borrow_and_update());
                                },
                                else => {break;}
                            }
                        }
                    });
                }

                apalis_imx8.register_adc_reader(ApalisIMX8ADC::ADC0);

                Self {
                    backend,
                    nav_forward,
                    nav_backward,
                    nav_enter,
                    ..Default::default()
                }
            }
            Backend::Simulator => Self {
                backend,
                ..Default::default()
            },
        }
    }

    pub fn power_suspend(&self) {
        match &self.backend {
            #[cfg(feature = "apalis_imx8")]
            Backend::ApalisIMX8(_device) => {
                println!("Power suspend not yet implemented!");
                // device.power_suspend();
            }
            _ => println!("Simulator does not support `power_suspend`"),
        }
    }
}
