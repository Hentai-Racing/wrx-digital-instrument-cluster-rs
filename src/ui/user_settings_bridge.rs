use crate::application::settings::SettingsManager;
use crate::slint_generatedApp::{AccessibilitySettings, App, ApplicationState, GlobalThemeData};

use paste::paste;
use slint::{ComponentHandle, Weak};
use std::sync::{Arc, RwLock};

pub fn bridge_settings(handle_weak: Weak<App>, serdes_manager: Arc<RwLock<SettingsManager>>) {
    match slint::spawn_local(async_compat::Compat::new(async move {
        if let Ok(serdes_manager) = serdes_manager.read() {
            if let Err(e) = serdes_manager.loaded().wait_for(|loaded| *loaded).await {
                eprintln!("Failed to wait for serdes manager loading: {e}")
            }
        }

        macro_rules! bind {
            {$slint_global:ident.$param:tt <=> $root:ident.$($tail:tt)+} => {{paste!{
                if let Some(handle) = handle_weak.upgrade() {
                    let handle_weak = handle_weak.clone();
                    let root = $root.clone();
                    let g = handle.global::<$slint_global>();

                    if let Ok(root) = root.try_read() {
                        g.[<set_ $param>](root.$($tail)+.value().into());
                    }

                    g.[<on_update_ $param>](move |value| {
                        if let Ok(mut root) = root.try_write() {
                            root.$($tail)+.set_value(value);
                        }

                        let handle_copy = handle_weak.clone();
                        let root = root.clone();

                        if let Err(e) = handle_copy.upgrade_in_event_loop(move |handle| {
                            if let Ok(root) = root.try_read() {
                                let g = handle.global::<$slint_global>();
                                g.[<set_ $param>](root.$($tail)+.value().into());
                            }
                        }) {
                            eprintln!("Failed to update {}: {e:?}", stringify!($slint_global.$param))
                        };
                    });
                }
            }}};
        }

        bind!(ApplicationState.user_unit <=> serdes_manager.user_settings.general.unit_system);
        bind!(GlobalThemeData.current_theme <=> serdes_manager.user_settings.theme.selected_theme);
        bind!(AccessibilitySettings.animations_enabled <=> serdes_manager.user_settings.accessibility.animations_enabled);
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}
