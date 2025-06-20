use gpio_cdev::{Chip, EventRequestFlags, Line, LineRequestFlags};
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

pub struct ApalisIMX8 {}

pub enum ApalisIMX8GPIO {
    GPIO1(Chip),     // LSIO.GPIO0.IO08
    GPIO2(Chip),     // LSIO.GPIO0.IO09
    GPIO3(Chip),     // LSIO.GPIO0.IO12
    GPIO4(Chip),     // LSIO.GPIO0.IO13
    GPIO7(Chip),     // LSIO.GPIO3.IO26
    GPIO8(Chip),     // LSIO.GPIO3.IO09
    Wake1Mico(Chip), // LSIO.GPIO2.IO20
}

const GPIO1: u32 = 8;
// const GPIO2: u32 = 9;
// const GPIO3: u32 = 12;
// const GPIO4: u32 = 13;
// const GPIO7: u32 = 26;
// const GPIO8: u32 = 9;

impl ApalisIMX8 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn power_suspend(&self) {
        let MEMDIR = "/sys/power/mem_sleep";
        let STATEDIR = "/sys/power/state ";
        let IDLE = "s2idle";
        let DEEP = "deep";
        let MEM = "mem";

        #[cfg(debug_assertions)]
        match process::Command::new("echo")
            .args(["+10", ">", "/sys/class/rtc/rtc1/wakealarm"]) // auto wake after 10 seconds
            .spawn()
        {
            Ok(_) => {
                println!("Successfully set wake alarm")
            }
            _ => {}
        };

        match process::Command::new("echo")
            .args(["s2idle", ">", "/sys/power/mem_sleep"])
            .spawn()
        {
            Ok(_) => {
                println!("Successfully set s2idle")
            }
            _ => {}
        };

        match process::Command::new("echo")
            .args(["deep", ">", "/sys/power/state"])
            .spawn()
        {
            Ok(_) => {
                println!("Successfully set deep sleep")
            }
            _ => {}
        };
    }

    pub fn monitor_gpio1(&self) {
        let chip = Chip::new("/dev/gpiochip0");

        if let Ok(mut chip) = chip {
            if let Ok(line) = chip.get_line(GPIO1) {
                // while let Ok(event) = line.events(
                //     LineRequestFlags::INPUT,
                //     EventRequestFlags::FALLING_EDGE,
                //     env!("CARGO_PKG_NAME"),
                // ) {
                //     println!("{event:?}");
                // }
            }
        }
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
