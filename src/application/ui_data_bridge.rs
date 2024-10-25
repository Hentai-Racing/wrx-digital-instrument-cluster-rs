use crate::data::car_data::CarData;
use crate::slint_generatedAppWindow::*;
use slint::{ComponentHandle, Weak};
use tokio;

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

        slint::spawn_local(async_compat::Compat::new(async move {
            let mut engine_rpm = car_data.engine_rpm().watch();
            let mut odometer = car_data.odometer().watch();

            let binding = main_window.unwrap();
            let ui_cardata = binding.global::<SCarData>();

            loop {
                // todo: this may not be the best example of an event handler
                tokio::select! {
                    _ = engine_rpm.changed() => {
                        ui_cardata.set_engine_rpm(IDataParameter {
                            max_value: car_data.engine_rpm().max().into(),
                            min_value: car_data.engine_rpm().min().into(),
                            units: "RPM".into(),
                            value: engine_rpm.borrow_and_update().clone().into(),
                        });
                    },
                    _ = odometer.changed() => {
                        ui_cardata.set_odometer(FDataParameter {
                            max_value: car_data.odometer().max().into(),
                            min_value: car_data.odometer().min().into(),
                            units: "MI".into(),
                            value: odometer.borrow_and_update().clone().into(),
                        });
                    }
                }
            }
        }))
        .unwrap();
    }
}
