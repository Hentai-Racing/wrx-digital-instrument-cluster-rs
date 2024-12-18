use crate::data::data_parameter::DataParameter;
use crate::data::units::{Unit, UnitSystem};
use crate::wrx_2018::{self, EngineMtGear, Messages};
use paste::paste;
use socketcan::tokio::CanSocket;

macro_rules! param_max_min {
    ($car_data:ident, $msg:path, $param:ident) => {
        $car_data.$param().set_min(paste!($msg::[<$param:upper _MIN>]));
        $car_data.$param().set_max(paste!($msg::[<$param:upper _MAX>]));
    };
}

macro_rules! get_param_range {
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
///         <Unit>? [OverrideSetterFn]? param_name: type (= default)?,+
///     };+
/// }
///
/// impl CarData {
///     fn OverrideSetterFn(&mut self, input: &dyn Any) {
///         ...
///     }
/// }
///```
macro_rules! CarData {
    ( $($msg:ident => { $($(<$unit:path>)? $([$process_override:ident])? $param:ident: $type:tt $(= $init:expr)?),+ $(,)? } );+; ) => {
        #[derive(Clone, Default)]
        pub struct CarData {
            $($($param: DataParameter<$type>,)*)*
        }

        impl CarData {
            pub fn new() -> Self {
                use Unit::*;
                let mut car_data = Self {..Default::default()};

                $($(
                    $(car_data.$param.set_value($init);)? // allow for optional initial values
                    $(car_data.$param.set_units($unit(UnitSystem::SI));)? // allow for optional unit type
                    get_param_range!(car_data, wrx_2018::$msg => $param: $type); // set min and max values for number types
                )*)*

                car_data
            }

            fn process_message(&mut self, message: Messages) {
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

            $($(pub fn $param(&mut self) -> &mut DataParameter<$type> {
                &mut self.$param
            })*)*
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
        cruise_control_speed: u8,
    };

    EngineWarningLights => {
        oil_pressure_warning_light_enabled: bool = true,
        check_engine_light_enabled: bool = true,
    };

    Odometer => {
        <Distance> odometer: f32
    };

    BrakePedal => {
        <Speed> vehicle_speed: f32
    };

    StatusSwitches => {
        lowbeams_enabled: bool = true,
        handbrake_sw: bool = true,
        parking_lights_enabled: bool = true,
        highbeams_enabled: bool = true,
        reverse_sw: bool = true
    };

    Cluster => {
        fuel_level: f32,
        driver_seatbelt_warning_enabled: bool = true,
        passenger_seatbelt_warning_enabled: bool = true,
        left_turn_signal_enabled: bool = true,
        right_turn_signal_enabled: bool = true,
    };

    Cluster2 => {
        fog_lights_enabled: bool = true,
    };

    Cabin => {
        left_front_door_open: bool = true,
        right_front_door_open: bool = true,
        right_rear_door_open: bool = true,
        left_rear_door_open: bool = true,
        trunk_open: bool = true,
        headlight_dimmer_enabled: bool = true,
        dimmer_max_brightness_enabled: bool = true,
    };

    XxxMsg340 => {
        any_door_open: bool = true,
    };

    DriverRoadAssists => {
        hill_assist_enabled: bool = true,
        active_tq_vectoring_enabled: bool = true,
        traction_control_disabled: bool = true,
    };

    BsdRcta => {
        rcta_enabled: bool = true,
        bsd_left: bool = true,
        bsd_right: bool = true,
        rcta_left_adjacent: bool = true,
        rcta_left_approaching: bool = true,
        rcta_right_adjacent: bool = true,
        rcta_right_approaching: bool = true,
    };

    SrsStatus => {
        srs_warning_light_enabled: bool = true,
    };

    DimmerAndHood => {
        hood_closed: bool = false,
    };
);

impl CarData {
    pub async fn bridge_socketcan(&mut self, mut can_socket: CanSocket) {
        use crate::wrx_2018::Messages;
        use embedded_can::Frame;
        use futures::stream::StreamExt;

        while let Some(Ok(frame)) = can_socket.next().await {
            if let Ok(message) = Messages::from_can_message(frame.id(), frame.data()) {
                self.process_message(message)
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
