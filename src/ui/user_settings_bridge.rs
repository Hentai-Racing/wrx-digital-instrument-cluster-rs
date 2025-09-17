use crate::application::settings::SettingsManager;
use crate::slint_generatedApp::{
    AccessibilitySettings, App, ApplicationState, DebugSettings, GeneralSettings, GlobalThemeData,
};

use paste::paste;
use slint::{ComponentHandle, Weak};
use std::sync::{Arc, RwLock};

pub fn bridge(handle_weak: Weak<App>, settings_manager: Arc<RwLock<SettingsManager>>) {
    match slint::spawn_local(async_compat::Compat::new(async move {
        if let Ok(settings_manager) = settings_manager.read() {
            if let Err(e) = settings_manager.loaded().wait_for(|loaded| *loaded).await {
                eprintln!("Failed to wait for settings manager loading: {e}")
            }
        }

        macro_rules! bind {
            {$slint_global:ident.$param:tt <=> $root:ident.$($tail:tt)+} => {{paste!{
                if let Some(handle) = handle_weak.upgrade() {
                    let handle_weak = handle_weak.clone();
                    let root = $root.clone();
                    let g = handle.global::<$slint_global>();

                    if let Ok(root) = root.read() {
                        g.[<set_ $param>](root.$($tail)+.value().into());
                    }

                    g.[<on_update_ $param>](move |value| {
                        if let Ok(mut root) = root.write() {
                            root.$($tail)+.set_value(value);
                        }

                        let handle_copy = handle_weak.clone();
                        let root = root.clone();

                        if let Err(e) = handle_copy.upgrade_in_event_loop(move |handle| {
                            if let Ok(root) = root.read() {
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

        bind!(ApplicationState.user_unit <=> settings_manager.user_settings.general.unit_system);
        bind!(ApplicationState.simulation_running <=> settings_manager.session_settings.simulation_settings.simulation_running);
        bind!(ApplicationState.running_can <=> settings_manager.session_settings.can_settings.running_can);
        bind!(GeneralSettings.disable_hill_assist <=> settings_manager.user_settings.general.disable_hill_assist);
        bind!(GlobalThemeData.current_theme <=> settings_manager.user_settings.theme.selected_theme);
        bind!(AccessibilitySettings.animations_enabled <=> settings_manager.user_settings.accessibility.animations_enabled);
        bind!(AccessibilitySettings.accessible_switches <=> settings_manager.user_settings.accessibility.accessible_switches);
        bind!(DebugSettings.debug_highlights <=> settings_manager.session_settings.debug_session_settings.debug_highlights);
        bind!(DebugSettings.debug_overlay_enabled <=> settings_manager.session_settings.debug_session_settings.debug_overlay_enabled);
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}
