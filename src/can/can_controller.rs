extern crate socketcan;

use socketcan::{CanFdFrame, CanFrame, CanSocket, Socket};
use std::time::Duration;

pub struct CanReader {
    socket: CanSocket,
}

impl CanReader {
    pub fn new(iface: &str) -> Result<CanReader, Box<dyn std::error::Error>> {
        let socket = CanSocket::open(iface)?;
        Ok(CanReader { socket })
    }

    pub fn read_frame(&mut self) -> Result<socketcan::CanFrame, Box<dyn std::error::Error>> {
        let frame = self.socket.read_frame()?;
        Ok(frame)
    }

    pub fn set_read_timeout(
        &mut self,
        duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.set_read_timeout(duration)?;
        Ok(())
    }
}
