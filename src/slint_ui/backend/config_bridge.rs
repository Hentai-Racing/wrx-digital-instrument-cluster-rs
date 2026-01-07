use crate::application::user::UserConfig;
use crate::data::parameters::Node;
use crate::slint_generatedApp::{
    AccessibilitySettings, App, ApplicationState, DebugSettings, GeneralSettings, GlobalThemeData,
    HardwareBackendData, SettingsLayout, SettingsMenuItem, SettingsMenuItemType, SystemInfo,
};

use pastey::paste;
use slint::{ComponentHandle, Weak};
use tokio::select;

use std::sync::Arc;

pub fn bridge(handle_weak: Weak<App>, user_config: Arc<UserConfig>) {
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
        bind!(ApplicationState.user_unit <=> user_config.user.general.unit_system);
        bind!(ApplicationState.running_simulation <=> user_config.session.simulation.running_simulation);
        bind!(ApplicationState.running_can <=> user_config.session.can.running_can);
        bind!(GeneralSettings.disable_hill_assist <=> user_config.user.general.disable_hill_assist);
        bind!(GlobalThemeData.current_theme <=> user_config.user.theme.selected_theme);
        bind!(AccessibilitySettings.animations_enabled <=> user_config.user.accessibility.animations_enabled);
        bind!(AccessibilitySettings.accessible_switches <=> user_config.user.accessibility.accessible_switches);
        bind!(DebugSettings.debug_mode <=> user_config.session.debug.debug_mode);
        bind!(DebugSettings.debug_highlights <=> user_config.session.debug.debug_highlights);
        bind!(DebugSettings.debug_overlay_enabled <=> user_config.session.debug.debug_overlay_enabled);
        bind!(DebugSettings.extra_debug_info <=> user_config.session.debug.extra_debug_info);

        bind!(SystemInfo.total_memory <=| user_config.session.system_info.total_memory_mb);
        bind!(SystemInfo.used_memory <=| user_config.session.system_info.used_memory_mb);
        bind!(SystemInfo.process_memory <=| user_config.session.system_info.process_memory_mb);
        bind!(SystemInfo.process_memory_max <=| user_config.session.system_info.process_memory_max_mb);
        bind!(SystemInfo.num_cpus <=| user_config.session.system_info.num_cpus);
        bind!(SystemInfo.cpu_usage <=| user_config.session.system_info.cpu_usage);
        bind!(SystemInfo.fps <=| user_config.session.system_info.fps);
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}

fn unroll_layout(layout: &mut Vec<SettingsMenuItem>, node: Node, current_path: String) {
    match node {
        Node::HiddenParameter() => {}
        Node::ReadOnlyParameter { name, ty } => layout.push(SettingsMenuItem {
            name: name.into(),
            param_type: ty.into(),
            param_path: (current_path + name).into(),
            item_type: SettingsMenuItemType::ReadOnlyParameter,
        }),
        Node::Parameter { name, ty } => layout.push(SettingsMenuItem {
            name: name.into(),
            param_type: ty.into(),
            param_path: (current_path + name).into(),
            item_type: SettingsMenuItemType::Parameter,
        }),
        Node::Page { name, items } => {
            let path = current_path + &format!("{name}.");
            layout.push(SettingsMenuItem {
                name: name.into(),
                param_type: "".into(),
                param_path: (&path).into(),
                item_type: SettingsMenuItemType::Page,
            });

            for item in items {
                unroll_layout(layout, item, path.clone());
            }

            layout.push(SettingsMenuItem {
                name: "".into(),
                param_type: "".into(),
                param_path: path.into(),
                item_type: SettingsMenuItemType::PageEnd,
            });
        }
    }
}

pub fn settings_menu_builder(handle_weak: Weak<App>, user_config: Arc<UserConfig>) {
    if let Some(handle) = handle_weak.upgrade() {
        let ui_layout = handle.global::<SettingsLayout>();
        let handle_weak = handle_weak.clone();

        {
            let user_config = user_config.clone();
            ui_layout.on_get_page_layout(move || {
                if let Some(handle) = handle_weak.upgrade() {
                    let backend_layout = user_config.get_page_layout();
                    let mut unrolled_layout: Vec<SettingsMenuItem> = Vec::new();

                    unroll_layout(&mut unrolled_layout, backend_layout, String::new());

                    let ui_layout = handle.global::<SettingsLayout>();
                    ui_layout.set_layout(unrolled_layout.as_slice().into());
                }
            });
        }

        {
            let user_config = user_config.clone();
            let backend_layout = user_config.get_page_layout();

            ui_layout.on_set_by_path(move |path, value| {
                if let Node::Page { name, items: _ } = &backend_layout {
                    if let Some(path) = path.strip_prefix(&format!("{}.", *name)) {
                        user_config.set_by_path(path, &value.as_str());
                    }
                }
            });
        }

        {
            let user_config = user_config.clone();
            let backend_layout = user_config.get_page_layout();

            ui_layout.on_get_by_path(move |path| {
                let mut ret = "error".into();

                if let Node::Page { name, items: _ } = &backend_layout {
                    if let Some(path) = path.strip_prefix(&format!("{}.", *name)) {
                        ret = user_config.get_by_path(path).into();
                    }
                }

                ret
            });
        }
    }
}
