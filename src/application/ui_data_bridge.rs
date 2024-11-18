use crate::data::{car_data::CarData, units::UnitSystem};
use crate::slint_generatedAppWindow::*;
use paste::paste;
use slint::{ComponentHandle, Weak};

macro_rules! stype_number_instantiate {
    ($car_data:ident, $param:ident: $type:ident) => {
        $type {
            name: stringify!($param).into(),
            value: $car_data.$param().value().into(),
            unit_str: $car_data.$param().get_unit_short_str().into(),
            max: $car_data.$param().max().into(),
            min: $car_data.$param().max().into(),
            ..Default::default()
        }
    };
}

macro_rules! stype_instantiate {
    ($car_data:ident, $param:ident: SIDataParameter) => {
        stype_number_instantiate!($car_data, $param: SIDataParameter)
    };
    ($car_data:ident, $param:ident: SFDataParameter) => {
        stype_number_instantiate!($car_data, $param: SFDataParameter)
    };
    ($car_data:ident, $param:ident: $type:tt) => {
        $type {
            name: stringify!($param).into(),          // set name
            value: $car_data.$param().value().into(), // set initial value
            ..Default::default()
        }
    };
}

macro_rules! number_param_convertion_handle {
    ($car_data:ident, $ui_car_data:ident, $ui_suser_unit:ident, $sparam:ident, $param:ident: $type:ty) => {
        paste!(
            let mut sparam_clone = $sparam.clone();

            let unit_system: UnitSystem = $ui_suser_unit.get_user_unit().into();
            let units = $car_data.$param().units();

            let new_value: f64 = (*$param.borrow_and_update()).into();
            let units_converted = units.convert_system_to(unit_system);
            sparam_clone.value = units.convert_value_to(new_value, unit_system) as $type;
            sparam_clone.unit_str = units_converted.get_short_str().into();

            sparam_clone.min = units.convert_value_to($car_data.$param().min(), unit_system) as $type;
            sparam_clone.max = units.convert_value_to($car_data.$param().max(), unit_system) as $type;

            $ui_car_data.[<set_ $param>](sparam_clone);
        )
    };
}

macro_rules! param_convertion_handle {
    ($car_data:ident, $ui_car_data:ident, $ui_suser_unit:ident, $sparam:ident, $param:ident: SIDataParameter) => {
        number_param_convertion_handle!{$car_data, $ui_car_data, $ui_suser_unit, $sparam, $param: i32}
    };
    ($car_data:ident, $ui_car_data:ident, $ui_suser_unit:ident, $sparam:ident, $param:ident: SFDataParameter) => {
        number_param_convertion_handle!{$car_data, $ui_car_data, $ui_suser_unit, $sparam, $param: f32}
    };
    ($car_data:ident, $ui_car_data:ident, $ui_suser_unit:ident, $sparam:ident, $param:ident: $type:tt) => {
        paste!(
            let mut sparam_clone = $sparam.clone();
            sparam_clone.value = (*$param.borrow_and_update()).into();

            $ui_car_data.[<set_ $param>](sparam_clone);
        )
    };
}

// todo: make a round-robin system to stop higher freqency signals from bullying if that is possible
// this will involve creating our own "event" system which has a queue
macro_rules! bridge {
    ($($param:ident: $type:tt),+ $(,)? ) => {
        pub fn run(&mut self) {
            let main_window = self.main_window.clone();
            let mut car_data = self.car_data.clone();

            self.handle_unit_system();

            match slint::spawn_local(async_compat::Compat::new(async move {
                let window_binding = main_window.unwrap();
                let ui_car_data = window_binding.global::<SCarData>();
                let ui_suser_unit = window_binding.global::<SUserUnit>();

                $(
                    let mut $param = car_data.$param().watch();

                    paste!{
                        let [<sparam_ $param>] = stype_instantiate!{car_data, $param: $type};
                        param_convertion_handle!(car_data, ui_car_data, ui_suser_unit, [<sparam_ $param>], $param: $type);
                    }
                )*

                loop {
                    tokio::select! {
                        $(_ = $param.changed() => {
                            paste!{
                                param_convertion_handle!(car_data, ui_car_data, ui_suser_unit, [<sparam_ $param>], $param: $type);
                            }
                        },)*
                    }
                }
            }))
            {
                Err(e) => eprintln!("UIDataBridge failed with error: {e}"),
                _ => {}
            };
        }

        pub fn update_all(&self) {
            let mut car_data = self.car_data.clone();
            let window_binding = self.main_window.clone().unwrap();
            let ui_car_data = window_binding.global::<SCarData>();
            let ui_suser_unit = window_binding.global::<SUserUnit>();

            $(paste!{
                let mut $param = car_data.$param().watch();
                let [<sparam_ $param>] = stype_instantiate!{car_data, $param: $type};
                param_convertion_handle!(car_data, ui_car_data, ui_suser_unit, [<sparam_ $param>], $param: $type);
            })*
        }
    };
}

#[derive(Clone)]
pub struct UIDataBridge {
    main_window: Weak<AppWindow>,
    car_data: CarData,
}

impl UIDataBridge {
    pub fn new(main_window: Weak<AppWindow>, car_data: CarData) -> Self {
        Self {
            main_window,
            car_data,
        }
    }

    bridge! {
        engine_rpm: SIDataParameter,
        mt_gear: SStrDataParameter,
        vehicle_speed: SFDataParameter,
        odometer: SFDataParameter,
        lowbeams_enabled: SBDataParameter,
        right_turn_signal_enabled: SBDataParameter,
        left_turn_signal_enabled: SBDataParameter,
        handbrake_sw: SBDataParameter
    }

    pub fn handle_unit_system(&self) {
        let window_binding = (*self).main_window.clone().unwrap();
        let ui_suser_unit = window_binding.global::<SUserUnit>();

        let ui_binding = window_binding.as_weak().clone();

        let self_clone = self.clone();
        ui_suser_unit.on_update_user_unit(move |value: SUnitSystem| {
            let binding = ui_binding.unwrap();
            let ui_suser_unit = binding.global::<SUserUnit>();
            ui_suser_unit.set_user_unit(value);

            self_clone.update_all();
        });
    }
}

// Special type conversion implementations

use crate::can::messages::wrx_2018::EngineStatusMtGear;

impl Into<slint::SharedString> for EngineStatusMtGear {
    fn into(self) -> slint::SharedString {
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

impl Into<f64> for EngineStatusMtGear {
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
