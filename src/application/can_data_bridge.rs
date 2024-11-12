use crate::data::car_data::CarData;
use crate::wrx_2018::Messages;
use embedded_can::Frame;
use socketcan::tokio::CanSocket;

pub struct CanDataBridge {
    car_data: CarData,
    can_socket: CanSocket,
}

impl CanDataBridge {
    pub fn new(car_data: CarData, can_socket: CanSocket) -> Self {
        Self {
            car_data,
            can_socket,
        }
    }

    pub async fn read_can_frames(&mut self) {
        use futures::stream::StreamExt;

        while let Some(Ok(frame)) = self.can_socket.next().await {
            self.parse_can_frame(frame);
        }
    }

    fn parse_can_frame(&mut self, frame: impl Frame) {
        if let Ok(message) = Messages::from_can_message(frame.id(), frame.data()) {
            self.process_message(message)
        }
    }

    fn process_message(&mut self, message: Messages) {
        /// Takes the message enum and its signals and does the necessary binding to cardata
        macro_rules! signal_bridge {
            ( $($msg:path => { $($param:ident),* });* ; ) => {
                match message {
                    $($msg(sig) => {
                            $({self.car_data.$param().set_value(sig.$param())})*
                    })*
                    _ => {}
                }
            };
        }

        use Messages::*;
        signal_bridge!(
            EngineStatus => {engine_rpm, mt_gear};
            Odometer => {odometer};
            XxxMsg209 => {vehicle_speed};
            StatusSwitches => {lowbeams_enabled, handbrake_sw};
            XxxMsg640 => {left_turn_signal_enabled, right_turn_signal_enabled};
        );
    }
}
