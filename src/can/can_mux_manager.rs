// #![allow(unused)]
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use embedded_can::{Frame, Id};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum OBD2Service {
    CurrentData = 0x1,
    FreezeFrame = 0x2,
    StoredDTCs = 0x3,
    ClearDTCs = 0x4,
    TestResultsNonCan = 0x5,
    TestResultsCan = 0x6,
    PendingDTCs = 0x7,
    Control = 0x8,
    VehicleInformation = 0x9,
    PermanentDTCs = 0xA,
}

#[derive(Debug)]
pub enum MuxParseError {
    InvalidOBD2Service,
    ConsecutiveFrameMisaligned,
    ConsecutiveFrameNoPriorData,
    InvalidISOTPFrameType,
}

#[derive(Debug)]
pub enum MuxParseResult {
    None,
    ParseComplete,
    AwaitingBroadcastAck,
    AwaitingReceiveAck,
    ConsecutiveFrameContinue,
}

#[repr(u8)]
pub enum ISOTPFrameType {
    SingleFrame = 0x0,
    FirstFrame = 0x1,
    ConsecutiveFrame = 0x2,
    FlowControlFrame = 0x30,
}

#[derive(Debug, Clone)]
pub struct MuxPayload {
    demux_len: usize, // number of additional bytes required to complete the demux
    next_sequence: u8,
    data_complete: bool,
    data: Vec<u8>, // data to be demuxed
}

impl MuxPayload {
    pub fn new(
        demux_len: usize,
        next_sequence: u8,
        data_complete: bool,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            demux_len,
            next_sequence,
            data_complete,
            data: data.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct MuxId {
    can_id: Id,
    mode: OBD2Service,
    pid: u8,
}

impl MuxId {
    pub fn new(can_id: Id, mode: OBD2Service, pid: u8) -> Self {
        Self { can_id, mode, pid }
    }
}

#[derive(Default, Clone)]
pub struct MuxContext {
    pub mux_data: BTreeMap<MuxId, MuxPayload>,

    pub waiting_for_responce: bool,
}

impl MuxContext {
    #[allow(unused)]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse_frame(&mut self, frame: impl Frame) -> Result<MuxParseResult, MuxParseError> {
        let raw_id: u32 = match frame.id() {
            Id::Standard(raw) => raw.as_raw().into(),
            Id::Extended(raw) => raw.as_raw().into(),
        };

        if matches!(raw_id, 0x7E0 | 0x7DF | (0x7e1..=0x7ef)) {
            self.parse_isotp_frame(frame)
        } else {
            Ok(MuxParseResult::None)
        }
    }

    fn parse_isotp_frame(&mut self, frame: impl Frame) -> Result<MuxParseResult, MuxParseError> {
        let id = frame.id();
        let payload = frame.data();

        let protocol_control: u8 = (payload[0] >> 4).into();

        if let Ok(isotp_frame) = ISOTPFrameType::try_from(protocol_control) {
            self.waiting_for_responce = false; // need some sort of timeout also

            match isotp_frame {
                ISOTPFrameType::SingleFrame => {
                    let data_bytes: usize = (payload[0] & 0xF).into();
                    let service = payload[1] - 0x40;
                    let pid = payload[2];
                    let data = &payload[3..=data_bytes];
                    let mux_length = data.len();

                    if let Ok(service) = OBD2Service::try_from(service) {
                        let mux_id = MuxId::new(id, service, pid);
                        let mux_data = MuxPayload::new(mux_length, 0, true, data);

                        self.mux_data.insert(mux_id, mux_data);

                        self.perform_demux();

                        Ok(MuxParseResult::ParseComplete)
                    } else {
                        Err(MuxParseError::InvalidOBD2Service)
                    }
                }
                ISOTPFrameType::FirstFrame => {
                    let mux_length = ((payload[0] & 0xF) as usize) << 8 | (payload[1] as usize) - 7;
                    let service = payload[2] - 0x40;
                    let pid = payload[3];
                    let frame_index = payload[4];
                    let data = &payload[5..];

                    if let Ok(service) = OBD2Service::try_from(service) {
                        let mux_id = MuxId::new(id, service, pid);
                        let mux_payload = MuxPayload::new(mux_length, frame_index, true, data);

                        self.mux_data.insert(mux_id, mux_payload);

                        Ok(MuxParseResult::AwaitingBroadcastAck)
                    } else {
                        Err(MuxParseError::InvalidOBD2Service)
                    }
                }
                ISOTPFrameType::ConsecutiveFrame => {
                    println!("Received consecutive frame");
                    let frame_num = payload[0] & 0xF;

                    if let Some((mux_id, mux_payload)) = self.mux_data.iter_mut().next_back() {
                        if mux_id.can_id == id {
                            if mux_payload.next_sequence == frame_num {
                                mux_payload.next_sequence = (frame_num + 1) % 0x10;
                            } else {
                                println!(
                                    "Recieved frame out of order. Was expecting {}, got {frame_num}",
                                    mux_payload.next_sequence
                                );
                                // TODO: handle getting the incorrect sequence
                            }
                            let data = &payload[1..];

                            mux_payload.demux_len -= mux_payload.demux_len.min(data.len());
                            mux_payload.data.extend(data);

                            if mux_payload.demux_len == 0 {
                                mux_payload.data_complete = true;
                                self.perform_demux();

                                Ok(MuxParseResult::ParseComplete)
                            } else {
                                // TODO: implement ability to track number of ack'd bytes and send
                                // awaitingack again if the message is incomplete
                                Ok(MuxParseResult::ConsecutiveFrameContinue)
                            }
                        } else {
                            //? Is it possible for the other broadcaster(s) to be going through this at the same time with
                            //?  a different response address?
                            // If so this function needs to be modified to not use next_back, but maybe iterate backwards
                            // and get the last payload with the same id and has the frame_num match up properly

                            Err(MuxParseError::ConsecutiveFrameMisaligned)
                        }
                    } else {
                        Err(MuxParseError::ConsecutiveFrameNoPriorData)
                    }
                }
                ISOTPFrameType::FlowControlFrame => {
                    Ok(MuxParseResult::AwaitingReceiveAck)
                    // used mostly for receiving ack frames when sending data
                    // TODO: we're not sending data right now, so don't care about this
                }
            }
        } else {
            Err(MuxParseError::InvalidISOTPFrameType)
        }
    }

    fn perform_demux(&mut self) {
        // TODO: attach this data to DataParameter or something
        //
        self.mux_data.retain(|mux_id, mux_payload| {
            if mux_payload.data_complete {
                match mux_id.mode {
                    OBD2Service::VehicleInformation => match mux_id.pid {
                        // TODO: remove this and make it parameterized
                        0x00 => {
                            println!("S9 PIDS Supported:");

                            let bits = BitVec::<u8, Msb0>::from_vec(mux_payload.data.to_owned());

                            // let mut pids: BTreeMap<u32, bool> = BTreeMap::new();
                            for (i, bit) in bits.iter().enumerate() {
                                let pid = i as u32 + 1;
                                let value = *bit;
                                // pids.insert(pid, value);

                                print!("{pid:02X}: {value}; ")
                            }

                            println!("")
                        }
                        0x02 => {
                            let vin = String::from_utf8_lossy(&mux_payload.data).into_owned();
                            println!("VIN: {:?}", vin);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            };

            !mux_payload.data_complete
        });
        // for (mux_id, mux_payload) in &self.mux_data {

        // }
    }
}

pub struct ISOTPAckFrame {
    id: Id,
    data: [u8; 8],
}

impl ISOTPAckFrame {
    pub fn new(id: Id) -> Self {
        let mut data = [0u8; 8];
        data[0] = ISOTPFrameType::FlowControlFrame.into();
        // data[2] = 0xA0;

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
fn search_payload_unaligned(payload: &[u8], pattern: u64) -> bool {
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

// Trait Implementations

impl TryFrom<u8> for OBD2Service {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Self::CurrentData),
            0x2 => Ok(Self::FreezeFrame),
            0x3 => Ok(Self::StoredDTCs),
            0x4 => Ok(Self::ClearDTCs),
            0x5 => Ok(Self::TestResultsNonCan),
            0x6 => Ok(Self::TestResultsCan),
            0x7 => Ok(Self::PendingDTCs),
            0x8 => Ok(Self::Control),
            0x9 => Ok(Self::VehicleInformation),
            0xA => Ok(Self::PermanentDTCs),
            _ => Err(()),
        }
    }
}

impl Into<u8> for OBD2Service {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Eq for OBD2Service {}

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

impl Into<u8> for ISOTPFrameType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl std::fmt::Display for MuxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MuxParseError::ConsecutiveFrameMisaligned => {
                write!(f, "Consecutive frame is not the expected Id")
            }
            MuxParseError::ConsecutiveFrameNoPriorData => {
                write!(f, "Consecutive frame does not have prior data to append")
            }
            MuxParseError::InvalidISOTPFrameType => {
                write!(f, "Frame protocol control is not an ISO-TP type")
            }
            MuxParseError::InvalidOBD2Service => write!(f, "Invalid OBD2 service"),
        }
    }
}

impl std::error::Error for MuxParseError {}
