use crate::data::car_data::CarData;
use crate::slint_generatedAppWindow::*;
use paste::paste;
use slint::Weak;

// todo: make a round-robin system to stop higher freqency signals from bullying
// this will involve creating our own "event" system which has a queue
macro_rules! BridgeEvents {
    ($car_data:ident, $window:ident, $($name:ident),*) => {
        $(let mut $name = $car_data.$name().watch();)*

        loop {
            tokio::select! {
                $(_ = $name.changed() => {
                    paste!{$window.[<set_ $name>]((*$name.borrow_and_update()).into());}
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
            let window = main_window.unwrap();

            BridgeEvents!(
                car_data,
                window,
                //
                engine_rpm,
                mt_gear,
                vehicle_speed,
                odometer,
                lowbeams_enabled,
                right_turn_signal_enabled,
                left_turn_signal_enabled,
                handbrake_sw
            );
        })) {
            Err(e) => eprintln!("UIDataBridge failed with error: {e}"),

            _ => {}
        };
    }
}
