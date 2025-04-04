#![allow(unused)]

use std::collections::BTreeMap;

pub enum OBD2Modes {
    CurrentData,
    FreezeFrame,
    StoredDTCs,
    ClearDTCs,
    TestResultsNonCan,
    TestResultsCan,
    PendingDTCs,
    Control,
    VehicleInformation,
    PermanentDTCs,
}

pub struct MuxFrame {
    size: usize,
    mode: u8,
    pid: u8,
    data: Vec<u8>,
}

pub struct MuxRequest {
    mode: u8,
    pid: u8,
}

impl MuxFrame {
    pub fn new(size: usize, mode: u8, pid: u8, data: Vec<u8>) -> Self {
        Self {
            size,
            mode,
            pid,
            data,
        }
    }
}

struct MuxContext {
    request_queue: Vec<MuxRequest>,
    responses: BTreeMap<u8, MuxFrame>,
    running: bool,
}

impl MuxContext {
    pub fn new() -> Self {
        Self {
            request_queue: vec![],
            responses: BTreeMap::new(),
            running: false,
        }
    }
}
