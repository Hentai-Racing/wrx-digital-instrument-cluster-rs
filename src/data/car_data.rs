use crate::can::messages::wrx_2018::{self, EngineMtGear, Messages};
use crate::data::data_parameter::DataParameter;
use crate::data::units::{Unit, UnitSystem};

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
        $self.$process_override(&$sig.$param());
    };
}

/// Example:
///```
/// CarData! {
///     MessageEnum => {
///         <Unit(:UnitSystem)?>? [OverrideSetterFn]? param_name: type (= default)?,+
///         <Speed:USCS> [SpeedOverride] cruise_speed: u16 = 91,
///     };+
/// }
///
/// impl CarData {
///     (fn OverrideSetterFn(&mut self, input: &dyn Any) {
///         ...
///     })?
/// }
///```
///
/// Note: ```bool``` data types default to true unless otherwise stated
///
macro_rules! CarData {
    ( $($msg:ident => { $($(<$unit:path$(:$unit_system:path)?>)? $([$process_override:ident])? $param:ident: $type:tt $(= $init:expr)?),+ $(,)? } );+; ) => {
        #[derive(Clone, Default)]
        pub struct CarData {
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

CarData!(
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
        hood_closed: bool = false,
    };
);

fn search_payload_unaligned(payload: &[u8], pattern: u64) -> bool {
    let search_len = pattern.ilog2() + 1;
    let mut current = 0u64;

    for &byte in payload {
        for b in 0u8..8u8 {
            current = (current << 1) | ((byte >> (7 - b)) & 1) as u64;
            current &= (1 << search_len) - 1;

            if current == pattern {
                return true;
            }
        }
    }

    false
}

impl CarData {
    #[cfg(target_os = "linux")]
    pub async fn bridge_socketcan(&mut self, mut can_socket: socketcan::tokio::CanSocket) {
        use crate::wrx_2018::Messages;
        use embedded_can::Frame;
        use futures::stream::StreamExt;

        while let Some(Ok(frame)) = can_socket.next().await {
            let raw_id = frame.raw_id();
            let id = frame.id();
            let payload = frame.data();

            if let Ok(message) = Messages::from_can_message(id, payload) {
                self.process_message(&message)
            } else {
                #[cfg(debug_assertions)]
                match raw_id {
                    0x7e0 => {
                        let test_tpms = search_payload_unaligned(payload, 0x75B);
                        if test_tpms {
                            println!("Sent: {:?}", payload);
                        }
                    }
                    0x7e1..=0x7e8 => {
                        let test_tpms = search_payload_unaligned(payload, 0x75b);
                        if test_tpms {
                            println!("Recv: {:?}", payload);
                        }
                    }
                    _ => {}
                }
            }
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
