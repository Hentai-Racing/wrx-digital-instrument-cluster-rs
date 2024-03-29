slint::include_modules!();
mod can;
mod cardata;

use can::can_controller::CanController;
use cardata::CarData;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let mut car_data = CarData::new();

    ui.run()
}
#[cfg(target_os = "linux")]
fn can_init() {
    let can_controller = CanController::new("vcan0".to_string());
    can_controller.init().unwrap();
}

#[cfg(not(target_os = "linux"))]
fn can_init() {
    println!("This platform does not support socketcan. CAN controller will not be initialized.")
}
