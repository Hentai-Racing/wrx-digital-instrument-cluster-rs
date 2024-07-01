use socketcan::{CanSocket, Socket};

pub struct CanController {
    socket: CanSocket,
}

impl CanController {
    pub fn new(iface: &str) -> Result<CanController, Box<dyn std::error::Error>> {
        let socket = CanSocket::open(iface)?;
        // socket.set_nonblocking(true)?;
        Ok(CanController { socket })
    }

    pub fn read_frame(&mut self) -> Result<socketcan::CanFrame, Box<dyn std::error::Error>> {
        let frame = self.socket.read_frame()?;
        Ok(frame)
    }

    pub fn set_timeout(
        &mut self,
        timeout: std::time::Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.set_read_timeout(timeout)?;
        self.socket.set_write_timeout(timeout)?;
        Ok(())
    }
}
