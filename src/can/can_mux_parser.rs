// #![allow(unused)]
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use embedded_can::{Frame, Id};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
pub enum S9VehicleInformation {
    PIDs = 0x0,
    VIN = 0x02,
    ECU = 0x0A,
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
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
#[derive(Copy, Clone, Debug, TryFromPrimitive, IntoPrimitive)]
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
    category: DTCCategory,
    number: u16,
}

const DTC_SIZE: usize = 2;

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
            category: category.try_into().expect("infallible"),
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

#[repr(u8)]
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq)]
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
#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive, PartialEq, Eq)]
pub enum UDSService {
    DiagnosticSessionControl = 0x10,
    ECUReset = 0x11,
    SecurityAccess = 0x27,
    CommunicationControl = 0x28,
    Authentication = 0x29,
    TesterPresent = 0x3E,
    ControlDTCSettings = 0x85,
    ResponseOnEvent = 0x86,
    LinkControl = 0x87,

    ReadDataByIdentifier = 0x22,
    ReadMemoryByAddress = 0x23,
    ReadScalingDataByIdentifier = 0x24,
    ReadDataByPeriodicIdentifier = 0x2A,
    DynamicallyDefineDataIdentifier = 0x2C,
    WriteDataByIdentifier = 0x2E,
    WriteMemoryByAddress = 0x3D,

    ClearDiagnosticInformation = 0x14,
    ReadDTCInformation = 0x19,

    InputOutputControlByIdentifier = 0x2F,

    RoutineControl = 0x31,

    RequestDownload = 0x34,
    RequestUpload = 0x35,
    TransferData = 0x36,
    RequestTransferExit = 0x37,
    RequestFileTransfer = 0x38,

    SecuredDataTransmission = 0x84,

    /// 0x7F - 0x40
    NegativeResponce = 0x3F,
}

#[repr(u8)]
#[derive(Debug, IntoPrimitive)]
pub enum ISOTPFrameType {
    SingleFrame = 0x0,
    FirstFrame = 0x1,
    ConsecutiveFrame = 0x2,
    FlowControlFrame = 0x3,
}

#[allow(unused)]
#[repr(u8)]
pub enum FlowControlFlag {
    Continue = 0x0,
    Wait = 0x1,
    Abort = 0x2,
}

// #[allow(unused)]
#[derive(Debug)]
pub enum MuxParseError {
    UnknownMessageId,
    ConsecutiveFrameNoPriorData,
    /// ISOTP leaves an additional 2 bits for other frame types in the future
    InvalidISOTPFrameType,
}

#[derive(Debug)]
pub enum MuxParseResult {
    /// Message fully muxed
    ParseComplete,
    /// Requests from other broadcasters, not to be processed by us
    BroadcastFeedback,
    /// Waiting for our broadcaster to send a control frame
    AwaitingBroadcastAck,
    /// Waiting to receive a control from from bus
    AwaitingReceiveAck,
    /// Full mux is incomplete, continue until complete
    ConsecutiveFrameContinue,
}

#[derive(Debug)]
pub struct ISOTPMux {
    can_id: Id,
    /// number of additional bytes required to complete the demux
    demux_len: usize,
    /// frame index of next message in current mux
    next_sequence: usize,
    /// received all frames in transaction, data is fully muxed
    mux_complete: bool,
    /// data to be demuxed
    data: Vec<Vec<u8>>,
}

pub fn raw_id(id: Id) -> u32 {
    match id {
        Id::Standard(id) => id.as_raw() as u32,
        Id::Extended(id) => id.as_raw(),
    }
}

#[derive(Default)]
pub struct MuxContext {
    pub iso_tp_frames: Vec<ISOTPMux>,

    pub waiting_for_responce: bool,
}

impl MuxContext {
    #[allow(unused)]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse_frame(&mut self, frame: &impl Frame) -> Result<MuxParseResult, MuxParseError> {
        if matches!(raw_id(frame.id()), 0x7DF | (0x7E0..=0x7EF)) {
            self.parse_isotp_frame(frame)
        } else {
            Err(MuxParseError::UnknownMessageId)
        }
    }

    fn parse_isotp_frame(&mut self, frame: &impl Frame) -> Result<MuxParseResult, MuxParseError> {
        let id = frame.id();

        if (0x7DF..=0x7E0).contains(&raw_id(id)) {
            return Ok(MuxParseResult::BroadcastFeedback);
        }

        let payload = frame.data();
        let protocol_control: u8 = (payload[0] >> 4).into();

        if let Ok(isotp_frame) = ISOTPFrameType::try_from(protocol_control) {
            self.waiting_for_responce = false; // need some sort of timeout also

            match isotp_frame {
                ISOTPFrameType::SingleFrame => {
                    let mux_payload = ISOTPMux {
                        can_id: id,
                        demux_len: (payload[0] & 0xF) as usize,
                        data: vec![(&payload[1..]).to_vec()],
                        mux_complete: true,
                        next_sequence: 0,
                    };

                    self.iso_tp_frames.push(mux_payload);
                    self.demux_isotp();

                    Ok(MuxParseResult::ParseComplete)
                }
                ISOTPFrameType::FirstFrame => {
                    let mux_payload = ISOTPMux {
                        can_id: id,
                        demux_len: ((payload[0] & 0xF) as usize) << 8 | (payload[1] as usize),
                        data: vec![(&payload[2..]).to_vec()],
                        mux_complete: false,
                        next_sequence: 1,
                    };

                    self.iso_tp_frames.push(mux_payload);

                    Ok(MuxParseResult::AwaitingBroadcastAck)
                }
                ISOTPFrameType::ConsecutiveFrame => {
                    for mux_payload in self.iso_tp_frames.iter_mut().rev() {
                        let frame_index = (payload[0] & 0xF) as usize;

                        if mux_payload.can_id == frame.id() {
                            if mux_payload.next_sequence == frame_index {
                                mux_payload.next_sequence = (frame_index + 1) % 0x10;
                            } else {
                                continue;
                            }

                            let data = &payload[1..];

                            mux_payload.demux_len -= mux_payload.demux_len.min(data.len());
                            mux_payload.data.push(data.to_vec());

                            if mux_payload.demux_len == 0 {
                                mux_payload.mux_complete = true;
                                self.demux_isotp();

                                return Ok(MuxParseResult::ParseComplete);
                            } else {
                                return Ok(MuxParseResult::ConsecutiveFrameContinue);
                            }
                        }
                    }

                    Err(MuxParseError::ConsecutiveFrameNoPriorData)
                }
                ISOTPFrameType::FlowControlFrame => Ok(MuxParseResult::AwaitingReceiveAck), // TODO: this is functionally incomplete, but we are not acting as a reciever for the time being
            }
        } else {
            Err(MuxParseError::InvalidISOTPFrameType)
        }
    }

    fn demux_isotp(&mut self) {
        self.iso_tp_frames.retain(|isotp_payload| {
            if isotp_payload.mux_complete {
                let service = (isotp_payload.data[0][0] - 0x40) % 0x10;
                let pid = isotp_payload.data[0][1];
                let mut demuxed_data = (&isotp_payload.data[0][2..]).to_vec();

                if isotp_payload.data.len() > 0 {
                    for frame in &isotp_payload.data[1..] {
                        if service != (frame[0] - 0x40) {
                            break;
                        }
                        if pid != frame[0] {
                            // TODO: support multi-pid reponces. This is currently not done to keep the enum simple and not include each length
                            // the lengths are not sent in the message
                            // probably would implement this in the parse demux
                            break;
                        }
                        demuxed_data.extend(&frame[2..]);
                    }
                }

                Self::parse_demux(isotp_payload.can_id, service, pid, &demuxed_data);
            }
            !isotp_payload.mux_complete
            // if mux_payload.data_complete {

            // }
        });
    }

    fn parse_demux(can_id: Id, service: u8, pid: u8, data: &[u8]) {
        // TODO: attach this data to DataParameter or something
        if let Ok(service) = OBDService::try_from(service) {
            match service {
                OBDService::VehicleInformation => {
                    if let Ok(vehicle_information) = S9VehicleInformation::try_from(pid) {
                        match vehicle_information {
                            // TODO: remove this and make it parameterized
                            S9VehicleInformation::PIDs => {
                                println!("S9 PIDS Supported:");

                                let bits = BitVec::<u8, Msb0>::from_vec(data.to_owned());

                                // let mut pids: BTreeMap<u32, bool> = BTreeMap::new();
                                for (i, bit) in bits.iter().enumerate() {
                                    let pid = i as u32 + 1;
                                    let value = *bit;
                                    // pids.insert(pid, value);

                                    if value {
                                        print!("[{pid:02X}] ")
                                    }
                                }

                                println!();
                            }
                            S9VehicleInformation::VIN => {
                                let vin = String::from_utf8_lossy(data).into_owned();
                                println!("VIN: {:?}", vin);
                            }
                            S9VehicleInformation::ECU => {
                                let ecu = String::from_utf8_lossy(data).into_owned();
                                println!("ECU: {:?}", ecu);
                            }
                        }
                    }
                }
                OBDService::CurrentData => {
                    if let Ok(current_data) = S1CurrentData::try_from(pid) {
                        match current_data {
                            S1CurrentData::PIDs1
                            | S1CurrentData::PIDs2
                            | S1CurrentData::PIDs3
                            | S1CurrentData::PIDs4
                            | S1CurrentData::PIDs5
                            | S1CurrentData::PIDs6
                            | S1CurrentData::PIDs7 => {
                                const LEN: usize = 4;

                                println!("S1 PIDS Supported:");

                                let bits = BitVec::<u8, Msb0>::from_vec(data.to_owned());

                                // let mut pids: BTreeMap<u32, bool> = BTreeMap::new();
                                for (i, bit) in bits.iter().enumerate() {
                                    let pid = i as u32 + 1;
                                    let value = *bit;
                                    // pids.insert(pid, value);

                                    if value {
                                        print!("[{pid:02X}] ")
                                    }
                                }

                                println!();
                            }
                            S1CurrentData::EngineLoad => {
                                const LEN: usize = 1;
                                let value = data[0] as f32 * (100f32 / 255f32);
                                println!("Calculated engine load: {value}");
                            }
                            S1CurrentData::ControlModuleVoltage => {
                                const LEN: usize = 2;

                                let value = ((data[0] as u16) << 8) | (data[1] as u16);
                                let _voltage = value as f32 * 0.001;
                            }
                            S1CurrentData::EngineFuelRate => {
                                const LEN: usize = 2;

                                let value = ((data[0] as u16) << 8) | (data[1] as u16);
                                // L/h
                                let _rate = value as f32 / 20f32;
                            }
                            _ => {}
                        }
                    }
                }
                OBDService::StoredDTCs => {
                    let (chunks, _remainder) = data.as_chunks::<DTC_SIZE>();

                    for &chunk in chunks {
                        let dtc = DTC::from(chunk);
                        println!("{dtc}")
                    }

                    let remaining_bytes = _remainder.len();
                    if remaining_bytes > 0 {
                        println!(
                            "WARNING: {remaining_bytes} remaining bytes in DTC payload--should be 0"
                        )
                    }
                }
                // OBDService::NegativeResponce => println!("Recieved negative responce from ECU. Did you send the correct query?"),
                _ => {
                    println!(
                        "Demuxed complete OBD2 data | Id: {:?}, Mode: {:02X}, Pid: {pid:02X}: {data:02X?}",
                        raw_id(can_id),
                        service as u8
                    );
                }
            }
        } else if let Ok(service) = UDSService::try_from(service) {
            println!("Demuxed complete data in UDS format. Currently unimplemented.");
            println!(
                "{:02X} | Service: {:02X}, PID: {pid:02X}: {data:02X?}",
                raw_id(can_id),
                service as u8
            )
        } else {
            println!(
                "Demuxed complete data in unknown format | {:02X} | Service: {:02X}, PID: {pid:02X}: {data:02X?}",
                raw_id(can_id),
                service as u8
            );
        }
    }
}

pub struct ISOTPAckFrame {
    id: Id,
    data: [u8; 8],
}

// TODO: implement num frames and timing
impl ISOTPAckFrame {
    pub fn new(id: Id) -> Self {
        let mut data = [0u8; 8];
        data[0] = (ISOTPFrameType::FlowControlFrame as u8) << 4;
        data[0] |= FlowControlFlag::Continue as u8;
        // data[1] = num_frames
        // data[2] = frame_timing_ms;

        Self { id, data }
    }
}

impl Frame for ISOTPAckFrame {
    fn new(id: impl Into<Id>, _data: &[u8]) -> Option<Self> {
        Some(Self::new(id.into()))
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }

    fn id(&self) -> Id {
        self.id
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        unimplemented!()
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }
}

/// use to search for known patterns in unknown signals
/// TODO: just use bitvec
#[allow(unused)]
pub fn search_payload_unaligned(payload: &[u8], pattern: u64) -> bool {
    let search_len = pattern.ilog2() + 1;
    let mut current = 0u64;

    for &byte in payload {
        for b in 0u8..8u8 {
            current = (current << 1) | ((byte >> (7 - b)) & 1) as u64;
            current &= (1 << search_len) - 1;

            if current == pattern {
                return true;
            }
        }
    }

    false
}

impl TryFrom<u8> for ISOTPFrameType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::SingleFrame),
            0x1 => Ok(Self::FirstFrame),
            0x2 => Ok(Self::ConsecutiveFrame),
            0x3 => Ok(Self::FlowControlFrame),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for MuxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConsecutiveFrameNoPriorData => {
                write!(f, "Consecutive frame does not have prior data to append")
            }
            Self::InvalidISOTPFrameType => {
                write!(f, "Frame protocol control is not an ISO-TP type")
            }
            Self::UnknownMessageId => write!(f, "Frame is not a known mux id"),
        }
    }
}

impl std::error::Error for MuxParseError {}
