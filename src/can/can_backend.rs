#![allow(unused)] // temporary while this is being implemented

use embedded_can::{Frame, Id};
use std::error::Error;
use std::path::Path;

use serial;
#[cfg(target_os = "linux")]
use socketcan;

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
    interface: CanInterface,
    socket: Option<CanSocket>,
}

impl CanBackend {
    pub fn new(
        interface_type: SelectedCanInterface,
        interface_path: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let interface: Option<CanInterface> = match interface_type {
            #[cfg(target_os = "linux")]
            SelectedCanInterface::VirtualCan => {}
            #[cfg(target_os = "linux")]
            SelectedCanInterface::Can => {}
            #[cfg(feature = "slcan")]
            SelectedCanInterface::SerialCan => {
                match serial::SystemPort::open(Path::new(interface_path)) {
                    Ok(port) => Some(CanInterface::Serial(port)),
                    Err(e) => {
                        eprintln!("Error opening serial device {interface_path}: {e}");
                        None
                    }
                }
            }
            _ => return Err("Interface unsupported".into()),
        };

        if let Some(interface) = interface {
            Ok(Self {
                interface,
                socket: None,
            })
        } else {
            Err("No interface".into())
        }
    }

    pub fn initialize_hardware(&self) -> Result<(), Box<dyn Error>> {
        match &self.interface {
            #[cfg(target_os = "linux")]
            CanInterface::Can(interface) => {}
            #[cfg(feature = "slcan")]
            CanInterface::Serial(_interface) => Ok(()),
        }
    }

    pub fn initialize_socket(mut self) -> Result<(), Box<dyn Error>> {
        match self.interface {
            #[cfg(target_os = "linux")]
            CanInterface::Can(interface) => {}
            #[cfg(feature = "slcan")]
            CanInterface::Serial(interface) => {
                let mut socket = slcan::CanSocket::<serial::SystemPort>::new(interface);

                match socket.close() {
                    Ok(_) => match socket.open(slcan::BitRate::Setup500Kbit) {
                        Ok(_) => {
                            self.socket = Some(CanSocket::Serial(socket));
                            Ok(())
                        }
                        Err(e) => Err(e.into()),
                    },
                    Err(e) => Err(e.into()),
                }
            }
        }
    }

    // pub fn read_frame(self) -> Result<CanFrame, Box<dyn Error>> {
    //     if let Some(socket) = self.socket {
    //         match socket {
    //             #[cfg(target_os = "linux")]
    //             CanSocket::SocketCan(_) => {}
    //             #[cfg(feature = "slcan")]
    //             CanSocket::Serial(mut socket) => match socket.receive() {
    //                 Ok(frame) => Ok(CanFrame::from_frame(frame)),
    //                 Err(e) => Err(format!("Failed to read frame {e:?}").into()),
    //             },
    //         }
    //     } else {
    //         Err("No socket configured".into())
    //     }
    // }
}
