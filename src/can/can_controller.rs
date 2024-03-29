use socketcan::Socket;
use anyhow::Result;

pub struct CanController {
	socket_name: String,
}

impl CanController {
	pub fn new(socket_name: String) -> CanController {
		CanController {
			socket_name,
		}
	}

	pub fn init(&self) -> Result<()> {
		let socket = socketcan::CanSocket::open(&self.socket_name.as_str())?;
		socket.set_nonblocking(true)?;

		println!("CAN Controller initialized");

		Ok(())
	}
}
