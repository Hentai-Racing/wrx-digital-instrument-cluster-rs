use embedded_can::Frame;
use embedded_can::Id;
use socketcan::tokio::CanSocket;
use socketcan::CanFrame;
use socketcan::StandardId;
// use std::ops::RangeInclusive;

#[derive(Default, Clone)]
pub struct CanMessageHandler {
    pid_sent: u8,
    service_sent: u8,
    querying: bool,
}

impl CanMessageHandler {
    const OBD0_ID: u16 = 0x7DF; // Primary OBD2 ID
                                // const OBD1_ID: Range<u16> = 0x7E8; // Secondary OBD2 ID

    // const OBD_RESPONCE_RANGE: RangeInclusive<i32> = 0x7E8..=0x7EF;

    // byte format of obd2 message
    // [additional bytes 2][service][PID][]
    // byte format of response
    // [additional bytes 3..=6][service + 0x40H][PID][data][data][data][0x00 or 0x55H]

    pub fn new() -> Self {
        Self {
            querying: false,
            ..Default::default()
        }
    }

    pub fn process_message(&mut self, frame: CanFrame) {
        let id = frame.id();
        let data = frame.data();

        match id {
            Id::Standard(id) => {
                let id_num = id.as_raw();

                match id_num {
                    Self::OBD0_ID => {}
                    0x7E8..=0x7EF => {
                        let mut str: String = "[".into();
                        for i in data {
                            str.push_str(&format!(" 0x{:02X} ", i));
                        }
                        str += "]";

                        println!(
                            "{id_num:03X}: {frame:?}, [{:02X}, {:02X}]",
                            self.pid_sent, self.service_sent
                        );

                        // if (data[2] as i32 - 0x40) == self.pid_sent as i32 {
                        //     println!("Good")
                        // }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub async fn bridge_socketcan(&mut self, mut can_socket: CanSocket) {
        use futures::stream::StreamExt;

        while let Some(Ok(frame)) = can_socket.next().await {
            self.process_message(frame);
        }
    }

    pub fn request_vin(&mut self, can_socket: &mut CanSocket) {
        if !self.querying {
            // self.querying = true;

            let id = Id::Standard(unsafe { StandardId::new_unchecked(Self::OBD0_ID) });
            let data: &[u8] = &[2, 0x09, 2, 0, 0, 0, 0, 0];
            let frame = CanFrame::new(id, data);

            if let Some(frame) = frame {
                match can_socket.write_frame(frame) {
                    Ok(fut) => {
                        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

                        let vcan_handle = tokio_runtime.spawn(async move {
                            match fut.await {
                                Ok(_) => {
                                    println!("Requested service 0x09 PID 2");
                                }
                                _ => {
                                    println!("Failed to request service 0x09 PID 2");
                                }
                            }
                        });

                        self.pid_sent = data[2];
                        self.service_sent = data[1];
                    }
                    _ => {}
                };
            }
        }
    }
}
