extern crate socketcan;

use socketcan::{CanFrame, CanSocket, Socket};
use std::time::Duration;

pub struct CanController {
    socket: CanSocket,
}

impl CanController {
    pub fn new(iface: &str) -> Result<CanController, Box<dyn std::error::Error>> {
        let socket = CanSocket::open(iface)?;
        Ok(CanController { socket })
    }

    pub fn read_frame(&mut self) -> Result<socketcan::CanFrame, Box<dyn std::error::Error>> {
        let frame = self.socket.read_frame()?;
        Ok(frame)
    }

    pub fn write_frame(&mut self, frame: CanFrame) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.write_frame(&frame)?;
        Ok(())
    }

    pub fn set_read_timeout(
        &mut self,
        duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.set_read_timeout(duration)?;
        Ok(())
    }
}
