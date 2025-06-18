use crate::application::serdes::SerdesManager;
use crate::slint_generatedApp::{App, ApplicationState, GlobalThemeData};

use slint::{ComponentHandle, Weak};
use std::sync::{Arc, RwLock};

pub fn bridge_settings(ui: Weak<App>, serdes_manager: Arc<RwLock<SerdesManager>>) {
    match slint::spawn_local(async_compat::Compat::new(async move {
        if let Ok(serdes_manager) = serdes_manager.read() {
            if let Err(e) = serdes_manager.loaded().wait_for(|loaded| *loaded).await {
                eprintln!("Failed to wait for serdes manager loading: {e}")
            }
        }

        if let Some(ui) = ui.upgrade() {
            let ui_binding = ui.as_weak();

            let application_state = ui.global::<ApplicationState>();
            let global_theme_data = ui.global::<GlobalThemeData>();

            if let Ok(serdes_manager) = serdes_manager.read() {
                global_theme_data.set_current_theme(
                    serdes_manager
                        .user_settings
                        .theme
                        .selected_theme
                        .value()
                        .into(),
                );

                application_state.set_user_unit(
                    serdes_manager
                        .user_settings
                        .general
                        .unit_system
                        .value()
                        .into(),
                );
            }

            {
                let serdes_manager = serdes_manager.clone();

                application_state.on_update_user_unit(move |value| {
                    if let Ok(mut serdes_manager) = serdes_manager.write() {
                        serdes_manager
                            .user_settings
                            .general
                            .unit_system
                            .set_value(value);
                    }

                    let serdes_manager = serdes_manager.clone();
                    if let Err(e) = ui_binding.upgrade_in_event_loop(move |ui| {
                        if let Ok(serdes_manager) = serdes_manager.read() {
                            let application_state = ui.global::<ApplicationState>();
                            application_state.set_user_unit(
                                serdes_manager
                                    .user_settings
                                    .general
                                    .unit_system
                                    .value()
                                    .into(),
                            );
                        }
                    }) {
                        eprintln!("Failed to update application state: {e:?}")
                    }
                });
            }

            {
                let serdes_manager = serdes_manager.clone();
                global_theme_data.on_theme_changed(move |value| {
                    if let Ok(mut serdes_manager) = serdes_manager.write() {
                        serdes_manager
                            .user_settings
                            .theme
                            .selected_theme
                            .set_value(value);
                    }
                });
            }
        }
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}
