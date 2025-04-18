use crate::can::can_mux_manager::{MuxContext, MuxParseError, MuxParseResult};
use crate::can::messages::wrx_2018::{self, EngineMtGear, Messages};
use crate::data::data_parameter::DataParameter;
use crate::data::units::{Unit, UnitSystem};

use embedded_can::Frame;
use paste::paste;

macro_rules! param_max_min {
    ($car_data:ident, $msg:path, $param:ident) => {paste!(
        $car_data.$param().set_min($msg::[<$param:upper _MIN>]);
        $car_data.$param().set_max($msg::[<$param:upper _MAX>]);
    )};
}

macro_rules! bool_default {
    ($car_data:ident, $msg:path, $param:ident) => {
        $car_data.$param().set_value(true);
    };
}

use UnitSystem::*;
macro_rules! unit_system_type_overload {
    ($type:expr) => {
        $type
    };
    () => {
        SI
    };
}

macro_rules! handle_param_type {
    ($car_data:ident, $msg:path => $param:ident: f32) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: f64) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: u8) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: u16) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: u32) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: u64) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: i8) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: i16) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: i32) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: i64) => {
        param_max_min!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: bool) => {
        bool_default!($car_data, $msg, $param)
    };
    ($car_data:ident, $msg:path => $param:ident: $type:ty) => {};
}

macro_rules! HandleSignalProcess {
    ($self:ident, $sig:ident, $param:ident) => {
        $self.$param().set_value($sig.$param());
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
/// Note: ```bool``` data types default to true unless otherwise stated
///
macro_rules! CarData {
    { {$( $visible:vis $struct_param:ident: $struct_param_ty:ty ),+}; $($msg:ident => { $($(<$unit:path$(:$unit_system:path)?>)? $([$process_override:ident])? $param:ident: $type:tt $(= $init:expr)?),+ $(,)? } );+; } => {
        #[derive(Clone, Default)]
        pub struct CarData {
            $($visible $struct_param: $struct_param_ty,)+
            $($($param: DataParameter<$type>,)*)*
        }

        impl CarData {
            pub fn new() -> Self {
                use Unit::*;
                let mut car_data = Self {
                    ..Default::default()
                };

                $($(
                    handle_param_type!(car_data, wrx_2018::$msg => $param: $type); // set min and max values for number types
                    $(car_data.$param.set_value($init);)? // allow for optional initial values
                    $(car_data.$param.set_units($unit(unit_system_type_overload!($($unit_system)?)));)? // allow for optional unit type
                )*)*

                car_data
            }

            pub fn process_message(&mut self, message: &Messages) {
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
                pub fn $param(&mut self) -> &mut DataParameter<$type> {
                    &mut self.$param
                }
            )*)*
        }
    }
}

// TODO: make a hashmap or something so all the parameters can be displayed in slint as a list

CarData! {
    {
        pub obd_mux_context: MuxContext
    };

    Engine => {
        engine_rpm: u16,
        mt_gear: EngineMtGear = EngineMtGear::Neutral
    };

    EngineStatus2 => {
        <Temperature> engine_oil_temp: i16,
        <Temperature> engine_coolant_temp: i16,
        cruise_control_enabled: bool,
        cruise_control_set_enabled: bool,
        /* <Speed:USCS> */ cruise_control_speed: u8,
    };

    EngineWarningLights => {
        oil_pressure_warning_light_enabled: bool,
        check_engine_light_enabled: bool,
    };

    Odometer => {
        <Distance> odometer: f32
    };

    BrakePedal => {
        <Speed> vehicle_speed: f32
    };

    StatusSwitches => {
        lowbeams_enabled: bool,
        handbrake_sw: bool,
        parking_lights_enabled: bool,
        highbeams_enabled: bool,
        reverse_sw: bool
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
        rcta_enabled: bool,
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
    pub fn parse_frame(&mut self, frame: &impl Frame) -> Result<ParseResult, ParseError> {
        let data = &frame.data()[..frame.dlc()];

        match Messages::from_can_message(frame.id(), data) {
            Ok(message) => {
                self.process_message(&message);
                return Ok(ParseResult::Ok);
            }
            Err(e) => match e {
                // if it's an unknown message, we will handle it with a different parser
                wrx_2018::CanError::UnknownMessageId(_) => {}
                _ => return Err(ParseError::CanError(e)),
            },
        };

        match self.obd_mux_context.parse_frame(frame) {
            Ok(mux_result) => return Ok(ParseResult::Mux(mux_result)),
            Err(e) => return Err(ParseError::MuxError(e)),
        }
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
            None
        }
    }
}
