use crate::data::parameters::FieldParameter;
use crate::data::{car_data::CarData, units::UnitSystem};
use crate::slint_generatedApp::*;

use paste::paste;
use slint::{ComponentHandle, Weak};
use tokio::select;

macro_rules! number_param_convertion_handle {
    ($car_data:ident, $ui_car_data:ident, $unit_system:ident, $param:ident: $type:ty = $value:expr) => {paste!(
        let units = $car_data.$param().units();

        let mut sparam = $ui_car_data.[<get_ $param>]();
        sparam.value = units.convert_value_to($value as f64, $unit_system) as $type;
        sparam.min = units.convert_value_to($car_data.$param().min() as f64, $unit_system) as $type;
        sparam.max = units.convert_value_to($car_data.$param().max() as f64, $unit_system) as $type;
        sparam.unit_str = units.convert_system_to($unit_system).get_short_str().into();

        $ui_car_data.[<set_ $param>](sparam);
    )};
}

macro_rules! param_convertion_handle {
    ($car_data:ident, $ui_car_data:ident, $unit_system:ident, $param:ident: SIDataParameter = $value:expr) => {
        number_param_convertion_handle!{$car_data, $ui_car_data, $unit_system, $param: i32 = $value}
    };
    ($car_data:ident, $ui_car_data:ident, $unit_system:ident, $param:ident: SFDataParameter = $value:expr) => {
        number_param_convertion_handle!{$car_data, $ui_car_data, $unit_system, $param: f32 = $value}
    };
    ($car_data:ident, $ui_car_data:ident, $unit_system:ident, $param:ident: $type:tt = $value:expr) => {paste!(
        let mut sparam = $ui_car_data.[<get_ $param>]();
        sparam.value = $value.into();

        $ui_car_data.[<set_ $param>](sparam);
    )};
}

macro_rules! bridge {
    ($($param:ident: $type:tt),+ $(,)? ) => {
        pub fn run(&mut self) {
            // self.handle_unit_system();

            $(
                let ui = self.ui.clone();
                let car_data = self.car_data.clone();
                let mut unit_system_changed = self.unit_system_parameter.watch();
                let mut thread_watch = car_data.$param().watch();

                slint::spawn_local(async_compat::Compat::new(async move {
                    if let Some(ui) = ui.upgrade() {
                        let ui_car_data = ui.global::<SCarData>();
                        let mut _unit_system: UnitSystem = *unit_system_changed.borrow_and_update();

                        loop { // do-while for initial setting of values, then wait for update events
                            let value = *thread_watch.borrow_and_update();
                            param_convertion_handle!(car_data, ui_car_data, _unit_system, $param: $type = value);

                            select! {
                                biased; // always check the unit system first
                                Ok(_) = unit_system_changed.changed() => {
                                    _unit_system = *unit_system_changed.borrow_and_update();
                                },
                                Ok(_) = thread_watch.changed() => {},
                                else => {
                                    // if for any reason one of the watches errors (by being dropped early), break the loop to stop deadlock
                                    // this should never happen, but we cannot not break the entire application if it does
                                    break;
                                },
                            };
                        }
                    }
                })).unwrap();
            )*
        }
    };
}

#[derive(Clone)]
pub struct SCarDataBridge {
    ui: Weak<App>,
    car_data: CarData,
    unit_system_parameter: FieldParameter<UnitSystem>,
}

impl SCarDataBridge {
    pub fn new(
        ui: Weak<App>,
        car_data: CarData,
        unit_system_parameter: FieldParameter<UnitSystem>,
    ) -> Self {
        let this = Self {
            ui,
            car_data,
            unit_system_parameter,
        };

        this
    }

    // TODO: auto generate this somewhere else, maybe in CarData itself
    bridge! {
        engine_rpm: SIDataParameter,
        mt_gear: SStrDataParameter,

        engine_oil_temp: SIDataParameter,
        engine_coolant_temp: SIDataParameter,
        engine_boost_pressure: SFDataParameter,
        cruise_control_enabled: SBDataParameter,
        cruise_control_set_enabled: SBDataParameter,
        cruise_control_speed: SIDataParameter,

        oil_pressure_warning_light_enabled: SBDataParameter,
        check_engine_light_enabled: SBDataParameter,

        odometer: SFDataParameter,

        vehicle_speed: SFDataParameter,

        lowbeams_enabled: SBDataParameter,
        handbrake_sw: SBDataParameter,
        parking_lights_enabled: SBDataParameter,
        highbeams_enabled: SBDataParameter,
        reverse_sw: SBDataParameter,

        fuel_level: SFDataParameter,
        driver_seatbelt_warning_enabled: SBDataParameter,
        passenger_seatbelt_warning_enabled: SBDataParameter,
        left_turn_signal_enabled: SBDataParameter,
        right_turn_signal_enabled: SBDataParameter,

        mt_clutch_sw: SBDataParameter,

        fog_lights_enabled: SBDataParameter,
        left_front_door_open: SBDataParameter,
        right_front_door_open: SBDataParameter,
        right_rear_door_open: SBDataParameter,
        left_rear_door_open: SBDataParameter,
        trunk_open: SBDataParameter,
        headlight_dimmer_enabled: SBDataParameter,
        dimmer_max_brightness_enabled: SBDataParameter,

        any_door_open: SBDataParameter,

        hill_assist_enabled: SBDataParameter,
        active_tq_vectoring_enabled: SBDataParameter,
        traction_control_disabled: SBDataParameter,

        rcta_enabled: SBDataParameter,
        rcta_left: SBDataParameter,
        rcta_right: SBDataParameter,
        bsd_left_adjacent: SBDataParameter,
        bsd_left_approaching: SBDataParameter,
        bsd_right_adjacent: SBDataParameter,
        bsd_right_approaching: SBDataParameter,

        srs_warning_light_enabled: SBDataParameter,

        hood_open: SBDataParameter,
    }
}

// Special type conversion implementations

use crate::can::messages::wrx_2018::EngineMtGear;

impl Into<slint::SharedString> for EngineMtGear {
    fn into(self) -> slint::SharedString {
        match &self {
            EngineMtGear::Floating => " ".into(),
            EngineMtGear::Neutral => "N".into(),
            EngineMtGear::X1 => "1".into(),
            EngineMtGear::X2 => "2".into(),
            EngineMtGear::X3 => "3".into(),
            EngineMtGear::X4 => "4".into(),
            EngineMtGear::X5 => "5".into(),
            EngineMtGear::X6 => "6".into(),
            _ => "?ERR_MT_GEAR".into(),
        }
    }
}

impl Into<UnitSystem> for SUnitSystem {
    fn into(self) -> UnitSystem {
        match self {
            Self::USCS => UnitSystem::USCS,
            Self::SI => UnitSystem::SI,
        }
    }
}

impl Into<SUnitSystem> for UnitSystem {
    fn into(self) -> SUnitSystem {
        match self {
            Self::USCS => SUnitSystem::USCS,
            Self::SI => SUnitSystem::SI,
        }
    }
}
