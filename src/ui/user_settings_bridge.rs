use crate::application::serdes::SerdesManager;
use crate::slint_generatedApp::{App, ApplicationState, GlobalThemeData};

use slint::{ComponentHandle, Weak};
use std::sync::{Arc, RwLock};

pub fn bridge_settings(ui: Weak<App>, serdes_manager: Arc<RwLock<SerdesManager>>) {
    match slint::spawn_local(async_compat::Compat::new(async move {
        match serdes_manager.read() {
            Ok(serdes_manager) => match serdes_manager.loaded().wait_for(|loaded| *loaded).await {
                Err(e) => eprintln!("Failed to wait for serdes manager loading: {e}"),
                _ => {}
            },
            _ => {}
        }

        if let Some(ui) = ui.upgrade() {
            let application_state = ui.global::<ApplicationState>();
            let global_theme_data = ui.global::<GlobalThemeData>();

            match serdes_manager.read() {
                Ok(serdes_manager) => {
                    global_theme_data.set_current_theme(
                        (&serdes_manager.user_settings.theme.selected_theme).into(),
                    );

                    application_state.invoke_update_user_unit(
                        serdes_manager.user_settings.general.unit_system.into(),
                    );
                }
                _ => {}
            }

            {
                let serdes_manager = serdes_manager.clone();
                application_state.on_update_user_unit(move |value: _| {
                    match serdes_manager.write() {
                        Ok(mut serdes_manager) => {
                            serdes_manager.user_settings.general.unit_system = value.into()
                        }
                        _ => {}
                    }
                });
            }

            {
                let serdes_manager = serdes_manager.clone();
                global_theme_data.on_theme_changed(move |value: _| match serdes_manager.write() {
                    Ok(mut serdes_manager) => {
                        serdes_manager.user_settings.theme.selected_theme = value.into();
                    }
                    _ => {}
                });
            }
        }
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}
