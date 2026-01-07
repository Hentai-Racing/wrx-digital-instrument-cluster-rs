use crate::application::user::UserConfig;
use crate::data::parameters::Node;
use crate::slint_generatedApp::{
    AccessibilitySettings, App, ApplicationState, DebugSettings, GeneralSettings, GlobalThemeData,
    HardwareBackendData, SystemInfo,
};

use pastey::paste;
use slint::{ComponentHandle, Weak};
use tokio::select;

use std::sync::Arc;

pub fn bridge(handle_weak: Weak<App>, config_manager: Arc<UserConfig>) {
    match slint::spawn_local(async_compat::Compat::new(async move {
        macro_rules! bind {
            {$slint_global:ident.$param:tt <=> $root:ident.$($tail:tt)+} => {{paste!{
                let handle_weak = handle_weak.clone();
                let root = $root.clone();

                if let Some(handle) = handle_weak.upgrade() {
                    let g = handle.global::<$slint_global>();

                    g.[<on_update_ $param>](move |value| {
                        root.$($tail)+.set_value(value.into());
                    });
                }

                bind!($slint_global.$param <=| $root.$($tail)+);
            }}};

            {$slint_global:ident.$param:tt <=| $root:ident.$($tail:tt)+} => {{paste!{
                let root = $root.clone();
                if let Some(handle) = handle_weak.upgrade() {
                    let _= slint::spawn_local(async_compat::Compat::new(async move {
                        let g = handle.global::<$slint_global>();
                        let mut watch = root.$($tail)+.watch();

                        loop {
                            g.[<set_ $param>](root.$($tail)+.value().into());

                            select!{
                                Ok(_) = watch.changed() => {},
                                else => {break;}
                            }
                        }
                    }));
                }
            }}};
        }

        // TODO: auto-generate entire settings menu and bindings
        // The plan is to add a sort of reflection system to `parameter_struct` and
        // read it in slint to dynamically generate the menues
        bind!(ApplicationState.user_unit <=> config_manager.user.general.unit_system);
        bind!(ApplicationState.running_simulation <=> config_manager.session.simulation.running_simulation);
        bind!(ApplicationState.running_can <=> config_manager.session.can.running_can);
        bind!(GeneralSettings.disable_hill_assist <=> config_manager.user.general.disable_hill_assist);
        bind!(GlobalThemeData.current_theme <=> config_manager.user.theme.selected_theme);
        bind!(AccessibilitySettings.animations_enabled <=> config_manager.user.accessibility.animations_enabled);
        bind!(AccessibilitySettings.accessible_switches <=> config_manager.user.accessibility.accessible_switches);
        bind!(DebugSettings.debug_mode <=> config_manager.session.debug.debug_mode);
        bind!(DebugSettings.debug_highlights <=> config_manager.session.debug.debug_highlights);
        bind!(DebugSettings.debug_overlay_enabled <=> config_manager.session.debug.debug_overlay_enabled);

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

pub fn settings_menu_builder(handle_weak: Weak<App>, config_manager: Arc<UserConfig>) {
    // TODO: give slint a format we can use to build a menu system
    if let Some(handle) = handle_weak.upgrade() {
        let layout = config_manager.get_page_layout();

        fn layout_unwrap(node: Node) {
            match node {
                Node::Page { name, items } => {}
                Node::Parameter(name) => {}
                Node::ReadOnlyParameter(name) => {}
            }
        }
    }
}
