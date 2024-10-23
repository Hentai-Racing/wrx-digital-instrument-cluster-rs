use crate::can::messages::wrx_2018;
use crate::data::data_parameter::DataParameter;

#[derive(Clone)]
pub struct CarData {
    engine_rpm: DataParameter<u16>,
    odometer: DataParameter<f32>,
}

impl CarData {
    pub fn new() -> Self {
        Self {
            engine_rpm: DataParameter::new(
                wrx_2018::EngineStatus::ENGINE_RPM_MIN,
                wrx_2018::EngineStatus::ENGINE_RPM_MAX,
            ),
            odometer: DataParameter::new(
                wrx_2018::Odometer::ODOMETER_MIN,
                wrx_2018::Odometer::ODOMETER_MAX,
            ),
        }
    }

    pub fn engine_rpm(&mut self) -> &mut DataParameter<u16> {
        &mut self.engine_rpm
    }

    pub fn odometer(&mut self) -> &mut DataParameter<f32> {
        &mut self.odometer
    }
}
