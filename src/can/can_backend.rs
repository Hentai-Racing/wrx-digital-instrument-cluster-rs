#![allow(unused)] // temporary while this is being implemented

use embedded_can::{Frame, Id};
use std::error::Error;
use std::path::Path;

use serial;
#[cfg(target_os = "linux")]
use socketcan::Socket;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SelectedCanInterface {
    VirtualCan,
    Can,
    SerialCan,
}

pub enum CanInterface {
    #[cfg(target_os = "linux")]
    SocketCan(socketcan::CanInterface),
    #[cfg(feature = "slcan")]
    Serial(serial::SystemPort),
}

pub enum CanSocket {
    #[cfg(target_os = "linux")]
    SocketCan(socketcan::CanSocket),
    #[cfg(feature = "slcan")]
    Serial(slcan::CanSocket<serial::SystemPort>),
}

pub struct CanFrame {
    id: Id,
    dlc: usize,
    data: [u8; 8],
}

impl CanFrame {
    pub fn new(id: Id, dlc: usize, data: &[u8]) -> Self {
        let mut buffer = [0u8; 8];
        let len = data.len().min(8);
        buffer[..len].copy_from_slice(&data[..len]);

        Self {
            id,
            dlc,
            data: buffer,
        }
    }

    pub fn from_frame(frame: impl Frame) -> Self {
        Self::new(frame.id(), frame.dlc(), frame.data())
    }
}

impl Frame for CanFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        Some(CanFrame::new(id.into(), data.len(), data))
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        None
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

pub struct CanBackend {
    socket: CanSocket,
}

impl CanBackend {
    pub fn new(
        interface_type: SelectedCanInterface,
        interface_path: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let socket: Option<CanSocket> = match interface_type {
            #[cfg(target_os = "linux")]
            SelectedCanInterface::VirtualCan | SelectedCanInterface::Can => {
                let mut created_interface = false;
                let can_if_type = if interface_type == SelectedCanInterface::VirtualCan {
                    "vcan"
                } else {
                    "can"
                };

                println!("Interface name: {interface_path}; Type: {can_if_type}");

                let interface = match socketcan::CanInterface::open(&interface_path) {
                    Ok(can_interface) => Some(can_interface),
                    _ => {
                        match socketcan::CanInterface::create(&interface_path, None, can_if_type) {
                            Ok(can_interface) => {
                                created_interface = true;
                                println!("Created CAN interface {interface_path}");
                                Some(can_interface)
                            }
                            Err(e) => {
                                eprintln!("Failed to create CAN interface {interface_path}: {e}");
                                None
                            }
                        }
                    }
                };

                if let Some(interface) = interface {
                    let can_bitrate = 500000;
                    let details = interface.details()?;

                    let is_up = if (&details).is_up {
                        true
                    } else {
                        interface.set_bitrate(can_bitrate, None)?;
                        match interface.bring_up() {
                            Ok(_) => true,
                            Err(e) => {
                                eprintln!(
                                    "Failed to bring up interface {}: {e:?}",
                                    details.name.unwrap()
                                );
                                false
                            }
                        }
                    };

                    if is_up {
                        let socket = socketcan::CanSocket::open_iface(details.index)?;

                        Some(CanSocket::SocketCan(socket))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            #[cfg(feature = "slcan")]
            SelectedCanInterface::SerialCan => {
                match serial::SystemPort::open(Path::new(interface_path)) {
                    Ok(port) => {
                        let mut socket = slcan::CanSocket::<serial::SystemPort>::new(port);

                        socket.close()?;
                        socket.open(slcan::BitRate::Setup500Kbit)?;

                        Some(CanSocket::Serial(socket))
                    }
                    Err(e) => {
                        eprintln!("Error opening serial device {interface_path}: {e}");
                        None
                    }
                }
            }

            _ => return Err("Interface unsupported".into()),
        };

        if let Some(socket) = socket {
            Ok(Self { socket })
        } else {
            Err("No socket".into())
        }
    }

    pub fn read_frame(&mut self) -> Option<CanFrame> {
        match self.socket {
            #[cfg(target_os = "linux")]
            CanSocket::SocketCan(ref socket) => {
                if let Ok(frame) = socket.read_frame() {
                    return Some(CanFrame::from_frame(frame));
                }
            }
            #[cfg(feature = "slcan")]
            CanSocket::Serial(ref mut socket) => {
                if let Ok(frame) = socket.read() {
                    return Some(CanFrame::from_frame(frame));
                }
            }
        }

        None
    }

    pub fn write_frame(&mut self, frame: impl Frame) -> Result<(), Box<dyn std::error::Error>> {
        match self.socket {
            #[cfg(target_os = "linux")]
            CanSocket::SocketCan(ref socket) => {
                let socketcan_frame = socketcan::CanFrame::new(frame.id(), frame.data());
                if let Some(frame) = socketcan_frame {
                    match socket.write_frame(&frame) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.into()),
                    }
                } else {
                    Err("Failed to create socketcan frame".into())
                }
            }
            #[cfg(feature = "slcan")]
            CanSocket::Serial(ref mut socket) => match socket.write(frame.id(), &frame.data()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            },
        }
    }
}
