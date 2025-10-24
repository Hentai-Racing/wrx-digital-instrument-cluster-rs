use crate::can::can_mux_manager::{MuxContext, MuxParseError, MuxParseResult};
use crate::can::messages::wrx_2018::{self, EngineMtGear, Messages};
use crate::data::parameters::DataParameter;
#[allow(unused_imports)]
use crate::data::units::Unit::{self, *};
#[allow(unused_imports)]
use crate::data::units::UnitSystem::{self, *};

use embedded_can::Frame;
#[allow(unused_imports)]
use pastey::paste;

macro_rules! default_value {
    ($ty:ty| $param_default:expr) => {
        $param_default.into()
    };
    (bool|) => {
        true.into()
    };
    ($ty:ty|) => {
        <$ty>::default().into()
    };
}

macro_rules! generate_param_unit_system {
    ($unit:path| $system:expr) => {
        Some($unit($system))
    };
    ($unit:path|) => {
        Some($unit(UnitSystem::default()))
    };
    () => {
        Default::default()
    };
}

macro_rules! generate_param_instantiation {
    (number $msg:path => $param:ident: $type:ty $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {paste!{
        DataParameter::new($msg::[<$param:upper _MIN>], $msg::[<$param:upper _MAX>], Some(default_value!($type| $($default_value)?)), generate_param_unit_system!($($unit| $($system)?)?))
    }};
    ($msg:path => $param:ident: $type:ty $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        default_value!($type| $($default_value)?)
    };
}

macro_rules! param {
    ($msg:path => $param:ident: f32 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: f32 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: f64 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: f64 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: u8 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: u8 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: u16 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: u16 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: u32 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: u32 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: u64 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: u64 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: i8 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: i8 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: i16 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: i16 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: i32 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: i32 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: i64 $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!(number $msg => $param: i64 $([$unit| $($system)?])? $(= $default_value)?)
    };
    ($msg:path => $param:ident: $type:tt $([$unit:path| $($system:expr)?])? $(= $default_value:expr)?) => {
        generate_param_instantiation!($msg => $param: $type $([$unit:path| $($system:expr)?])? $(= $default_value)?)
    };
}

macro_rules! HandleSignalProcess {
    ($self:ident, $sig:ident, $param:ident) => {
        $self.$param.set_value($sig.$param());
    };
    ($self:ident, $sig:ident, $param:ident, $process_override:ident) => {
        $self.$process_override(&$sig.$param(), $sig);
    };
}

/// Example:
///```
/// CarData! {
///     {
///         // normal struct stuff here
///     };
///
///     MessageEnum => {
///         <Unit(:UnitSystem)?>? [OverrideSetterFn]? param_name: type (= default)?,+
///         <Speed:USCS> [SpeedOverride] cruise_speed: u16 = 91,
///     };+
/// }
///
/// impl CarData {
///     (fn OverrideSetterFn(&mut self, input: &dyn Any, param: Message) {
///         ...
///     })?
/// }
///```
///
/// Note: ```bool``` data types default to `true` unless otherwise stated
///
macro_rules! CarData {
    { {$( $visible:vis $struct_param:ident: $struct_param_ty:tt $(= $struct_param_init:expr)?),*}; $($msg:ident => { $($(<$unit:path$(:$unit_system:path)?>)? $([$process_override:ident])? $param:ident: $type:tt $(= $init:expr)?),+ $(,)? } );+; } => {
        pub struct CarData {
            $($visible $struct_param: $struct_param_ty),*
            $($($param: DataParameter<$type>,)*)*
        }

        impl CarData {
            pub fn process_message(&self, message: &Messages) {
                match message {
                    $(
                        Messages::$msg(sig) => {
                        $(
                            HandleSignalProcess!(self, sig, $param $(, $process_override)?);
                        )*
                    })*
                    _ => {}
                }
            }

            $($(
                pub fn $param(&self) -> &DataParameter<$type> {
                    &self.$param
                }
            )*)*
        }

        impl Default for CarData {
            fn default() -> Self {
                Self {
                    $($struct_param: default_value!($struct_param_ty| $($struct_param_init)?)),*
                    $($($param: param!(wrx_2018::$msg => $param: $type $([$unit| $($unit_system)?])? $(= $init)?),)*)*
                }
            }
        }
    }
}

CarData! {
    {
        // pub obd_mux_context: MuxContext
    };

    Engine => {
        engine_rpm: u16,
        mt_gear: EngineMtGear,
    };

    EngineStatus2 => {
        <Temperature> engine_oil_temp: i16,
        <Temperature> engine_coolant_temp: i16,
        <Pressure> engine_boost_pressure: f32,
        cruise_control_enabled: bool,
        cruise_control_set_enabled: bool,
        /* <Speed:USCS> */ cruise_control_speed: u8,
    };

    EngineWarningLights => {
        oil_pressure_warning_light_enabled: bool,
        check_engine_light_enabled: bool,
    };

    Odometer => {
        <Distance> odometer: f32,
    };

    BrakePedal => {
        <Speed> vehicle_speed: f32,
    };

    StatusSwitches => {
        lowbeams_enabled: bool,
        handbrake_sw: bool,
        parking_lights_enabled: bool,
        highbeams_enabled: bool,
        reverse_sw: bool,
    };

    Cluster => {
        fuel_level: f32,
        driver_seatbelt_warning_enabled: bool,
        passenger_seatbelt_warning_enabled: bool,
        left_turn_signal_enabled: bool,
        right_turn_signal_enabled: bool,
    };

    MotorControl => {
        mt_clutch_sw: bool = false,
    };

    Cluster2 => {
        fog_lights_enabled: bool,
        tpms_warning_light_enabled: bool,
    };

    Cabin => {
        left_front_door_open: bool,
        right_front_door_open: bool,
        right_rear_door_open: bool,
        left_rear_door_open: bool,
        trunk_open: bool,
        headlight_dimmer_enabled: bool,
        dimmer_max_brightness_enabled: bool,
    };

    XxxMsg340 => {
        any_door_open: bool,
    };

    DriverRoadAssists => {
        hill_assist_enabled: bool,
        active_tq_vectoring_enabled: bool,
        traction_control_disabled: bool,
    };

    BsdRcta => {
        rcta_disabled: bool,
        rcta_left: bool,
        rcta_right: bool,
        bsd_left_adjacent: bool,
        bsd_left_approaching: bool,
        bsd_right_adjacent: bool,
        bsd_right_approaching: bool,
    };

    SrsStatus => {
        srs_warning_light_enabled: bool,
    };

    DimmerAndHood => {
        hood_open: bool,
    };
}

#[derive(Debug)]
pub enum ParseResult {
    Mux(MuxParseResult),
    Ok,
}

#[derive(Debug)]
pub enum ParseError {
    MuxError(MuxParseError),
    CanError(wrx_2018::CanError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MuxError(e) => e.fmt(f),
            Self::CanError(e) => e.fmt(f),
        }
    }
}

impl CarData {
    pub fn parse_frame(&self, frame: &impl Frame) -> Result<ParseResult, ParseError> {
        let data = &frame.data()[..frame.dlc()];

        match Messages::from_can_message(frame.id(), data) {
            Ok(message) => {
                self.process_message(&message);
                return Ok(ParseResult::Ok);
            }
            Err(e) => match e {
                // if it's an unknown message, we will handle it with a different parser
                // wrx_2018::CanError::UnknownMessageId(_) => {}
                _ => return Err(ParseError::CanError(e)),
            },
        };

        // match self.obd_mux_context.parse_frame(frame) {
        //     Ok(mux_result) => return Ok(ParseResult::Mux(mux_result)),
        //     Err(e) => return Err(ParseError::MuxError(e)),
        // }
    }
}

//
// EngineStatusMtGear implementations for DataParameter
//

impl Default for EngineMtGear {
    fn default() -> Self {
        EngineMtGear::Floating
    }
}

impl PartialOrd for EngineMtGear {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let v1 = u8::from(*self);
        let v2 = u8::from(*other);

        if v1 > v2 {
            Some(std::cmp::Ordering::Greater)
        } else if v1 < v2 {
            Some(std::cmp::Ordering::Less)
        } else if v1 == v2 {
            Some(std::cmp::Ordering::Equal)
        } else {
            std::option::Option::None
        }
    }
}
