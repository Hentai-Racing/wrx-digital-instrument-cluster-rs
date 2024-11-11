use crate::data::data_parameter::DataParameter;
use crate::wrx_2018::EngineStatusMtGear;

//todo: use syn to get all parameters from the message file like is done for virtual_can_generator in build.rs
macro_rules! CarData {
    ($($name:ident: $type:ty),*) => {
        #[derive(Clone, Default)]
        pub struct CarData {
            $($name: DataParameter<$type>,)*
        }

        impl CarData {
            pub fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }

            $(
                pub fn $name(&mut self) -> &mut DataParameter<$type> {
                    &mut self.$name
                }
            )*
        }
    }
}

CarData!(
    engine_rpm: u16,
    mt_gear: EngineStatusMtGear,
    vehicle_speed: f32,
    odometer: f32,
    lowbeams_enabled: bool,
    right_turn_signal_enabled: bool,
    left_turn_signal_enabled: bool,
    handbrake_sw: bool
);

//
// EngineStatusMtGear implementations for DataParameter
//

impl Default for EngineStatusMtGear {
    fn default() -> Self {
        EngineStatusMtGear::Floating
    }
}

impl PartialOrd for EngineStatusMtGear {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let v1: u8 = (*self).into();
        let v2: u8 = (*other).into();

        if v1 > v2 {
            Some(std::cmp::Ordering::Greater)
        } else if v1 < v2 {
            Some(std::cmp::Ordering::Less)
        } else if v1 == v2 {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl ToString for EngineStatusMtGear {
    fn to_string(&self) -> String {
        match &self {
            EngineStatusMtGear::Floating => " ".into(),
            EngineStatusMtGear::Neutral => "N".into(),
            EngineStatusMtGear::X1 => "1".into(),
            EngineStatusMtGear::X2 => "2".into(),
            EngineStatusMtGear::X3 => "3".into(),
            EngineStatusMtGear::X4 => "4".into(),
            EngineStatusMtGear::X5 => "5".into(),
            EngineStatusMtGear::X6 => "6".into(),
            _ => "?ERR_MT_GEAR".into(),
        }
    }
}

impl Into<slint::SharedString> for EngineStatusMtGear {
    fn into(self) -> slint::SharedString {
        self.to_string().into()
    }
}
