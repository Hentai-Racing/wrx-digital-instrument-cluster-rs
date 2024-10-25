use crate::data::car_data::CarData;
use crate::wrx_2018::Messages;
use embedded_can::Frame;
use socketcan::tokio::CanSocket;
use tokio::signal;

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
        match Messages::from_can_message(frame.id(), frame.data()) {
            Ok(message) => self.process_message(message),
            _ => {}
        }
    }

    fn process_message(&mut self, message: Messages) {
        match message {
            Messages::EngineStatus(signal) => {
                self.car_data.engine_rpm().set_value(signal.engine_rpm());
            }
            Messages::Odometer(signal) => {
                self.car_data.odometer().set_value(signal.odometer());
            }
            _ => {}
        }
    }
}
