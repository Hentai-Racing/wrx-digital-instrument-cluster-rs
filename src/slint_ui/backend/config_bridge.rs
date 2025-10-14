use crate::application::user::ConfigManager;
use crate::slint_generatedApp::{
    AccessibilitySettings, App, ApplicationState, DebugSettings, GeneralSettings, GlobalThemeData,
    HardwareBackendData, SystemInfo,
};

use pastey::paste;
use slint::{ComponentHandle, Weak};
use tokio::select;

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

            {$slint_global:ident.$param:tt <=| $root:ident.$($tail:tt)+} => {{paste!{
                let root = $root.clone();
                if let Some(handle) = handle_weak.upgrade() {
                    let _= slint::spawn_local(async_compat::Compat::new(async move {
                        let g = handle.global::<$slint_global>();
                        let mut watch = root.$($tail)+.watch();
                        g.[<set_ $param>](root.$($tail)+.value().into());


                        loop {
                            select!{
                            Ok(_) = watch.changed() => {
                                let value = *watch.borrow_and_update();
                                g.[<set_ $param>](value.into());
                            },
                            else => {break;}
                            }
                        }
                    }));
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

        bind!(SystemInfo.total_memory <=| config_manager.session.system_info.total_memory_mb);
        bind!(SystemInfo.used_memory <=| config_manager.session.system_info.used_memory_mb);
        bind!(SystemInfo.process_memory <=| config_manager.session.system_info.process_memory_mb);
        bind!(SystemInfo.process_memory_max <=| config_manager.session.system_info.process_memory_max_mb);
        bind!(SystemInfo.num_cpus <=| config_manager.session.system_info.num_cpus);
        bind!(SystemInfo.cpu_usage <=| config_manager.session.system_info.cpu_usage);
        bind!(SystemInfo.fps <=| config_manager.session.system_info.fps);
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}
