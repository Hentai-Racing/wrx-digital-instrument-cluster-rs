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
    ($car_data:ident, $ui_car_data:ident, $sparam:ident, $param:ident: $type:ty) => {
        paste!(
            let mut sparam_clone = $sparam.clone();

            let unit_system = UnitSystem::USCS;
            let units = $car_data.$param().units();
            let new_value: f64 = (*$param.borrow_and_update()).into();
            let converted_value = units.convert_to(new_value, unit_system);
            sparam_clone.min = units.convert_to($car_data.$param().min(), unit_system) as $type;
            sparam_clone.max = units.convert_to($car_data.$param().max(), unit_system) as $type;
            sparam_clone.value = converted_value as $type;

            $ui_car_data.[<set_ $param>](sparam_clone);
        )
    };
}

macro_rules! param_convertion_handle {
    ($car_data:ident, $ui_car_data:ident, $sparam:ident, $param:ident: SIDataParameter) => {
        number_param_convertion_handle!{$car_data, $ui_car_data, $sparam, $param: i32}
    };
    ($car_data:ident, $ui_car_data:ident, $sparam:ident, $param:ident: SFDataParameter) => {
        number_param_convertion_handle!{$car_data, $ui_car_data, $sparam, $param: f32}
    };
    ($car_data:ident, $ui_car_data:ident, $sparam:ident, $param:ident: $type:tt) => {
        paste!(
            let mut sparam_clone = $sparam.clone();
            sparam_clone.value = (*$param.borrow_and_update()).into();

            $ui_car_data.[<set_ $param>](sparam_clone);
        )
    };
}

// todo: make a round-robin system to stop higher freqency signals from bullying if that is possible
// this will involve creating our own "event" system which has a queue
macro_rules! event_bridge {
    (($car_data:ident, $ui_car_data:ident) => { $($param:ident: $type:tt),+ $(,)? }) => {
        $(
            let mut $param = $car_data.$param().watch();

            paste!{
                let [<sparam_ $param>] = stype_instantiate!{$car_data, $param: $type};
                $ui_car_data.[<set_ $param>]([<sparam_ $param>].clone());
            }
        )*

        loop {
            tokio::select! {
                $(_ = $param.changed() => {
                    paste!{
                        param_convertion_handle!($car_data, $ui_car_data, [<sparam_ $param>], $param: $type);
                    }
                },)*
            }
        }
    };
}

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

    pub fn run(&mut self) {
        let main_window = self.main_window.clone();
        let mut car_data = self.car_data.clone();

        match slint::spawn_local(async_compat::Compat::new(async move {
            let window_binding = main_window.unwrap();
            let ui_car_data = window_binding.global::<SCarData>();

            event_bridge!((car_data, ui_car_data) => {
                engine_rpm: SIDataParameter,
                mt_gear: SStrDataParameter,
                vehicle_speed: SFDataParameter,
                odometer: SFDataParameter,
                lowbeams_enabled: SBDataParameter,
                right_turn_signal_enabled: SBDataParameter,
                left_turn_signal_enabled: SBDataParameter,
                handbrake_sw: SBDataParameter
            });
        })) {
            Err(e) => eprintln!("UIDataBridge failed with error: {e}"),
            _ => {}
        };
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
