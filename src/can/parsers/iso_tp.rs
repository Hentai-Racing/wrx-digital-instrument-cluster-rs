use embedded_can::{Frame, Id};

#[repr(u8)]
#[derive(Debug)]
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

#[allow(unused)]
pub enum SeperationTime {
    /// milliseconds 0x00..=0x7F
    MS(u8),
    /// microseconds 0xf1..=0xf9
    US(u8),
}

#[derive(Debug)]
pub struct ISOTPMux {
    /// number of additional bytes required to complete the demux
    pub demux_len: usize,
    /// frame index of next message in current mux
    pub next_sequence: usize,
    /// received all frames in transaction, data is fully muxed
    pub mux_complete: bool,
    /// data to be demuxed
    pub data: Vec<u8>,
}

#[allow(unused)]
pub struct ISOTPControlFrame {
    pub frame_type: ISOTPFrameType,
    pub flow_control: FlowControlFlag,
    pub send_frames: u8,
    pub seperation_time: SeperationTime,
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
