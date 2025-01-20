use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::data::{car_data::CarData, units::UnitSystem};
use crate::slint_generatedAppWindow::*;
use paste::paste;
use slint::{ComponentHandle, Weak};

macro_rules! number_param_convertion_handle {
    ($car_data:ident, $ui_car_data:ident, $unit_system:ident, $param:ident: $type:ty = $value:expr) => {paste!(
        let units = $car_data.$param().units();

        let mut sparam = $ui_car_data.[<get_ $param>]();
        sparam.value = units.convert_value_to(f64::from($value), $unit_system) as $type;
        sparam.min = units.convert_value_to($car_data.$param().min(), $unit_system) as $type;
        sparam.max = units.convert_value_to($car_data.$param().max(), $unit_system) as $type;
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
            self.handle_unit_system();

            $(
                let main_window = self.main_window.clone();
                let mut car_data = self.car_data.clone();
                let unit_system_arc = Arc::clone(&self.unit_system);
                let mut thread_watch = car_data.$param().watch();

                slint::spawn_local(async_compat::Compat::new(async move {
                    if let Some(main_window) = main_window.upgrade() {
                        let ui_car_data = main_window.global::<SCarData>();

                        loop {
                            let value = *thread_watch.borrow_and_update();
                            let _unit_system: UnitSystem = unit_system_arc.load(Ordering::Relaxed).into();
                            param_convertion_handle!(car_data, ui_car_data, _unit_system, $param: $type = value);

                            if thread_watch.changed().await.is_err() {
                                break;
                            }
                        }
                    }
                })).unwrap();
            )*
        }

        pub fn update_all(&self) {
            if let Some(window_binding) = self.main_window.clone().upgrade() {
                let mut car_data = self.car_data.clone();
                let ui_car_data = window_binding.global::<SCarData>();
                let unit_system: UnitSystem = self.unit_system.load(Ordering::Relaxed).into();

                $(
                    let value = *car_data.$param().watch().borrow();
                    param_convertion_handle!(car_data, ui_car_data, unit_system, $param: $type = value);
                )*
            }
        }
    };
}

#[derive(Clone)]
pub struct SCarDataBridge {
    main_window: Weak<AppWindow>,
    car_data: CarData,
    unit_system: Arc<AtomicBool>,
}

impl SCarDataBridge {
    pub fn new(main_window: Weak<AppWindow>, car_data: CarData) -> Self {
        // uses an AtomicBool for the unitsystem because for the foreseable future
        // there are only two unitsystems we care about
        let ret = Self {
            main_window,
            car_data,
            unit_system: Arc::new(AtomicBool::new(UnitSystem::default().into())),
        };

        // set the initial unit system to whatever the UI is set to
        if let Some(window_binding) = ret.main_window.clone().upgrade() {
            let ui_application_state = window_binding.global::<ApplicationState>();
            let unit: UnitSystem = ui_application_state.get_user_unit().into();
            ret.unit_system.store(unit.into(), Ordering::Relaxed);
        }

        ret
    }

    pub fn handle_unit_system(&self) {
        if let Some(window_binding) = self.main_window.clone().upgrade() {
            let ui_application_state = window_binding.global::<ApplicationState>();
            let ui_binding = window_binding.as_weak().clone();
            let thread_self = self.clone();

            ui_application_state.on_update_user_unit(move |value: SUnitSystem| {
                if let Some(window_binding) = ui_binding.upgrade() {
                    let ui_application_state = window_binding.global::<ApplicationState>();
                    ui_application_state.set_user_unit(value);
                    let unit_system: UnitSystem = value.into();
                    thread_self
                        .unit_system
                        .store(unit_system.into(), Ordering::Relaxed);
                    thread_self.update_all();
                }
            });
        }
    }

    bridge! {
        engine_rpm: SIDataParameter,
        mt_gear: SStrDataParameter,

        engine_oil_temp: SIDataParameter,
        engine_coolant_temp: SIDataParameter,
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

        hood_closed: SBDataParameter,
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

impl std::fmt::Debug for EngineMtGear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gear_str: slint::SharedString = self.clone().into();
        write!(f, "{}", gear_str)
    }
}

impl Into<f64> for EngineMtGear {
    fn into(self) -> f64 {
        u8::from(self) as f64
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

impl Into<bool> for UnitSystem {
    fn into(self) -> bool {
        match self {
            Self::USCS => false,
            Self::SI => true,
        }
    }
}

impl Into<UnitSystem> for bool {
    fn into(self) -> UnitSystem {
        match self {
            false => UnitSystem::USCS,
            true => UnitSystem::SI,
        }
    }
}
