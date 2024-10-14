use std::collections::HashMap;

use crate::can::messages::wrx_2018;
use crate::data::data_parameter::DataParameter;

pub struct CarData {
    engine_rpm: DataParameter<u16>,
}

impl CarData {
    pub fn new() -> Self {
        Self {
            engine_rpm: DataParameter::<u16>::new(
                wrx_2018::EngineStatus::ENGINE_RPM_MIN,
                wrx_2018::EngineStatus::ENGINE_RPM_MAX,
            ),
        }
    }

    pub fn engine_rpm(&mut self) -> DataParameter<u16> {
        self.engine_rpm
    }
}
