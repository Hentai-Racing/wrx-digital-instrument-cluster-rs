use crate::application::user::ConfigManager;
use crate::slint_generatedApp::{
    AccessibilitySettings, App, ApplicationState, DebugSettings, GeneralSettings, GlobalThemeData,
    HardwareBackendData,
};

use pastey::paste;
use slint::{ComponentHandle, Weak};

use std::sync::Arc;

pub fn bridge(handle_weak: Weak<App>, config_manager: Arc<ConfigManager>) {
    match slint::spawn_local(async_compat::Compat::new(async move {
        macro_rules! bind {
            {$slint_global:ident.$param:tt <=> $root:ident.$($tail:tt)+} => {{paste!{
                if let Some(handle) = handle_weak.upgrade() {
                    let handle_weak = handle_weak.clone();
                    let root = $root.clone();
                    let g = handle.global::<$slint_global>();

                    g.[<set_ $param>](root.$($tail)+.value().into());

                    g.[<on_update_ $param>](move |value| {
                        root.$($tail)+.set_value(value.into());

                        let handle_copy = handle_weak.clone();
                        let root = root.clone();

                        if let Err(e) = handle_copy.upgrade_in_event_loop(move |handle| {
                            let g = handle.global::<$slint_global>();
                            g.[<set_ $param>](root.$($tail)+.value().into());
                        }) {
                            eprintln!("Failed to update {}: {e:?}", stringify!($slint_global.$param))
                        };
                    });
                }
            }}};
        }

        // TODO: auto-generate entire settings menu and bindings
        bind!(ApplicationState.user_unit <=> config_manager.user.general.unit_system);
        bind!(ApplicationState.simulation_running <=> config_manager.session.simulation.simulation_running);
        bind!(ApplicationState.running_can <=> config_manager.session.can.running_can);
        bind!(GeneralSettings.disable_hill_assist <=> config_manager.user.general.disable_hill_assist);
        bind!(GlobalThemeData.current_theme <=> config_manager.user.theme.selected_theme);
        bind!(AccessibilitySettings.animations_enabled <=> config_manager.user.accessibility.animations_enabled);
        bind!(AccessibilitySettings.accessible_switches <=> config_manager.user.accessibility.accessible_switches);
        bind!(DebugSettings.debug_highlights <=> config_manager.session.debug_session.debug_highlights);
        bind!(DebugSettings.debug_overlay_enabled <=> config_manager.session.debug_session.debug_overlay_enabled);
        bind!(HardwareBackendData.adc_val <=> config_manager.session.debug_hardware_backend_data.adc_val);
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}
