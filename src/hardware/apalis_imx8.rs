#![allow(unused)]
use crate::data::parameters::Parameter;

use gpio_cdev::{Chip, EventRequestFlags, EventType, LineRequestFlags};
use tokio::io::unix::AsyncFd;

use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process;
use std::sync::Arc;

#[repr(u8)]
#[derive(Clone)]
pub enum ApalisIMX8GPIO {
    GPIO1,     // LSIO.GPIO0.IO08
    GPIO2,     // LSIO.GPIO0.IO09
    GPIO3,     // LSIO.GPIO0.IO12
    GPIO4,     // LSIO.GPIO0.IO13
    GPIO7,     // LSIO.GPIO3.IO26
    GPIO8,     // LSIO.GPIO3.IO09
    Wake1Mico, // LSIO.GPIO2.IO20
    //
    __COUNT,
}

impl ApalisIMX8GPIO {
    pub const fn chip(&self) -> u32 {
        match self {
            Self::GPIO1 | Self::GPIO2 | Self::GPIO3 | Self::GPIO4 => 0,
            Self::Wake1Mico => 2,
            Self::GPIO7 | Self::GPIO8 => 3,
            //
            Self::__COUNT => unreachable!(),
        }
    }

    pub const fn line(&self) -> u32 {
        match self {
            Self::GPIO1 => 8,
            Self::GPIO2 => 9,
            Self::GPIO3 => 12,
            Self::GPIO4 => 13,
            Self::GPIO7 => 26,
            Self::GPIO8 => 9,
            Self::Wake1Mico => 20,
            //
            Self::__COUNT => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Clone)]
pub enum ApalisIMX8ADC {
    ADC1, // LSIO.GPIO3.IO18
    //
    __COUNT,
}

impl ApalisIMX8ADC {
    pub const fn chip(&self) -> u32 {
        match self {
            Self::ADC1 => 3,
            //
            Self::__COUNT => unreachable!(),
        }
    }

    pub const fn line(&self) -> u32 {
        match self {
            Self::ADC1 => 18,
            //
            Self::__COUNT => unreachable!(),
        }
    }
}

enum PowerState {
    IDLE,
    DEEP,
    MEM,
}

impl Into<&str> for PowerState {
    fn into(self) -> &'static str {
        match self {
            Self::IDLE => "s2idle",
            Self::DEEP => "deep",
            Self::MEM => "mem",
        }
    }
}

const MEMDIR: &str = "/sys/power/mem_sleep";
const STATEDIR: &str = "/sys/power/state ";
const WAKE_ALARM: &str = "/sys/class/rtc/rtc1/wakealarm";

#[derive(Default)]
pub struct ApalisIMX8 {
    gpios: [Arc<Parameter<bool>>; ApalisIMX8GPIO::__COUNT as usize],
    adcs: [Arc<Parameter<u32>>; ApalisIMX8ADC::__COUNT as usize],
}

impl ApalisIMX8 {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn power_suspend(&self) {
        #[cfg(debug_assertions)]
        match process::Command::new("echo")
            .args(["+10", ">", WAKE_ALARM]) // auto wake after 10 seconds
            .spawn()
        {
            Ok(_) => {
                println!("Successfully set wake alarm")
            }
            _ => {}
        };

        match process::Command::new("echo")
            .args([PowerState::IDLE.into(), ">", MEMDIR])
            .spawn()
        {
            Ok(_) => {
                println!("Successfully set s2idle")
            }
            _ => {}
        };

        match process::Command::new("echo")
            .args([PowerState::DEEP.into(), ">", STATEDIR])
            .spawn()
        {
            Ok(_) => {
                println!("Successfully set deep sleep")
            }
            _ => {}
        };
    }

    /*
        TODO: make special Parameter type for GPIO.
        TODO: implement GPIO writing
        TODO: implement ADC reading
        same for this
    */
    /*
       TODO: make a better method to get any gpio from an enum
       making the enums contain `Chip` is not likely to be useful to the caller
       the enum should only contain the directory(chip and line number)

       TODO: allow unregistering to not do extra kernel calls when not needed, or handling sleep states
    */
    pub fn register_gpio_reader(&self, gpio_pin: ApalisIMX8GPIO) {
        let param = self.gpios[gpio_pin.clone() as usize].clone();
        tokio::spawn(async move {
            let mut chip = match Chip::new(format!("/dev/gpiochip{}", gpio_pin.chip())) {
                Ok(chip) => chip,
                Err(e) => {
                    eprintln!("Chip error: {e}");
                    return;
                }
            };

            let line = match chip.get_line(gpio_pin.line()) {
                Ok(line) => line,
                Err(e) => {
                    eprintln!("Line error: {e}");
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
                    eprintln!("Event request error: {e}");
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
                            // TODO: debounce
                            Ok(event) => match event.event_type() {
                                EventType::RisingEdge => {
                                    param.set_value(true);
                                }
                                EventType::FallingEdge => {
                                    param.set_value(false);
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

    pub fn get_gpio_param(&self, gpio_pin: ApalisIMX8GPIO) -> Arc<Parameter<bool>> {
        self.gpios[gpio_pin as usize].clone()
    }
}

fn write_to_sysfs(path: &str, value: &str) {
    if let Ok(mut file) = OpenOptions::new().write(true).open(path) {
        match file.write_all(value.as_bytes()) {
            Err(e) => {
                eprintln!("Failed to write to sysfs file: {e:?}");
            }
            _ => {}
        };
    };
}
