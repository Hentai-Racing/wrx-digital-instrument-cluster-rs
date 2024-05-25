slint::include_modules!();
mod can;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use can::can_controller::CanReader;
use can::messages::wrx_2018;
use socketcan::CanFrame;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let mut controller = CanReader::new("can0").unwrap();
    controller.set_read_timeout(Duration::new(0, 5)).unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let handle = thread::spawn(move || {
        while running_clone.load(Ordering::SeqCst) {
            match controller.read_frame() {
                Ok(frame) => match frame {
                    CanFrame::Data(data) => {
                        let data_ref = data.as_ref();

                        let id = data_ref.can_id;
                        let dlc = data_ref.can_dlc;
                        let payload = data_ref.data;

                        let message = wrx_2018::Messages::from_can_message(id, &payload);

                        match message {
                            Ok(decoded_message) => match decoded_message {
                                wrx_2018::Messages::EngineStatus(signal) => {
                                    println!("{:?}", signal.engine_rpm());
                                }
                                _ => {}
                            },
                            Err(_) => {}
                        }
                    }
                    CanFrame::Error(_) => todo!(),
                    CanFrame::Remote(_) => todo!(),
                },
                Err(e) => eprintln!("Error reading frame: {}", e),
            }
        }
    });

    ui.run()?;

    // Stop the loop
    running.store(false, Ordering::SeqCst);

    // Wait for the thread to finish
    handle.join().unwrap();

    Ok(())
}
