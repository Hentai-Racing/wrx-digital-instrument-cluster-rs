use strum::FromRepr;

pub const SERVICE_OFFSET: u8 = 0x40;

#[repr(u8)]
#[derive(Debug, Clone, FromRepr, PartialEq, Eq)]
pub enum OBDService {
    CurrentData = 0x01,
    FreezeFrame = 0x02,
    StoredDTCs = 0x03,
    ClearDTCs = 0x04,
    TestResultsNonCan = 0x05,
    TestResultsCan = 0x06,
    PendingDTCs = 0x07,
    Control = 0x08,
    VehicleInformation = 0x09,
    PermanentDTCs = 0x0A,
}

#[repr(u8)]
#[derive(Debug, FromRepr)]
pub enum S1CurrentData {
    EngineLoad = 0x04,
    EngineSpeed = 0x0C,
    ControlModuleVoltage = 0x42,
    EngineFuelRate = 0x5E,
    Odometer = 0xA6,

    PIDs1 = 0x0,
    PIDs2 = 0x20,
    PIDs3 = 0x40,
    PIDs4 = 0x60,
    PIDs5 = 0x80,
    PIDs6 = 0xA0,
    PIDs7 = 0xC0,
}

#[repr(u8)]
#[derive(Debug, FromRepr)]
pub enum S9VehicleInformation {
    PIDs = 0x0,
    VIN = 0x02,
    ECU = 0x0A,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, FromRepr)]
pub enum DTCCategory {
    Powertrain = 0b00,
    Chassis = 0b01,
    Body = 0b10,
    Undefined = 0b11,
}

impl Into<char> for DTCCategory {
    fn into(self) -> char {
        match self {
            Self::Powertrain => 'P',
            Self::Chassis => 'C',
            Self::Body => 'B',
            Self::Undefined => 'U',
        }
    }
}

pub struct DTC {
    pub category: DTCCategory,
    pub number: u16,
}

pub const DTC_SIZE: usize = 2;

impl From<[u8; DTC_SIZE]> for DTC {
    fn from(value: [u8; DTC_SIZE]) -> Self {
        let category = value[0] >> 6;
        let mut number: u16 = (value[0] >> 4 & 0b11) as u16;

        for v in &value[0..] {
            number <<= 8;
            number += (v >> 4) as u16;
            number += (v & 0xF) as u16;
        }

        Self {
            category: DTCCategory::from_repr(category).expect("infallible"),
            number: number,
        }
    }
}

impl std::fmt::Display for DTC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:03X}",
            Into::<char>::into(self.category).to_uppercase(),
            self.number
        )
    }
}
