use gpio_cdev::{Chip, Line};
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

pub struct ApalisIMX8 {}

pub enum ApalisIMX8GPIO {
    GPIO1,     // LSIO.GPIO0.IO08
    GPIO2,     // LSIO.GPIO0.IO09
    GPIO3,     // LSIO.GPIO0.IO12
    GPIO4,     // LSIO.GPIO0.IO13
    GPIO7,     // LSIO.GPIO3.IO26
    GPIO8,     // LSIO.GPIO3.IO09
    Wake1Mico, // LSIO.GPIO2.IO20
}

// gpiodetect DOES NOT MATCH DOCUMENTATION
// gpio chip 2 line 8 gpio1 // MXM3_1/GPIO1
// gpio chip 2 line 9 gpio2
// gpio chip 2 line 12 gpio3
// gpio chip 2 line 13 gpio4
// gpio chip 5 line 26 gpio7
// gpio chip 5 line 28 gpio8 // gpio fan
// gpio chip 6 line 1 gpio5
// gpio chip 6 line 2 gpio6
// gpio chip 4 line 20 wake1mico

// GPIO5,     // LSIO.GPIO4.IO01 // flexcan2rx
// GPIO6,     // LSIO.GPIO4.IO02 // flexcan2tx

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
