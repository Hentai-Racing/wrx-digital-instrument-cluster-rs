use crate::application::config::Config;
use crate::data::parameters::Node;
use crate::slint_generatedApp::{
    AccessibilitySettings, App, ApplicationState, DebugSettings, GeneralSettings, GlobalThemeData,
    SettingsLayout, SettingsMenuItem, SettingsMenuItemType, SystemInfo,
};

use pastey::paste;
use slint::{ComponentHandle, Weak};
use tokio::select;

use std::rc::Rc;
use std::sync::Arc;

pub fn bridge(handle_weak: Weak<App>, config: Arc<Config>) {
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

        // TODO: auto-generate bindings
        bind!(ApplicationState.user_unit <=> config.user.general.unit_system);
        bind!(ApplicationState.running_simulation <=> config.session.simulation.running_simulation);
        bind!(ApplicationState.running_can <=> config.session.can.running_can);
        bind!(GeneralSettings.disable_hill_assist <=> config.user.general.disable_hill_assist);
        bind!(GlobalThemeData.current_theme <=> config.user.theme.selected_theme);
        bind!(AccessibilitySettings.animations_enabled <=> config.user.accessibility.animations_enabled);
        bind!(AccessibilitySettings.accessible_switches <=> config.user.accessibility.accessible_switches);
        bind!(DebugSettings.debug_mode <=> config.session.debug.debug_mode);
        bind!(DebugSettings.debug_highlights <=> config.session.debug.debug_highlights);
        bind!(DebugSettings.debug_overlay_enabled <=> config.session.debug.debug_overlay_enabled);
        bind!(DebugSettings.extra_debug_info <=> config.session.debug.extra_debug_info);

        bind!(SystemInfo.total_memory <=| config.session.system_info.total_memory_mb);
        bind!(SystemInfo.used_memory <=| config.session.system_info.used_memory_mb);
        bind!(SystemInfo.process_memory <=| config.session.system_info.process_memory_mb);
        bind!(SystemInfo.process_memory_max <=| config.session.system_info.process_memory_max_mb);
        bind!(SystemInfo.num_cpus <=| config.session.system_info.num_cpus);
        bind!(SystemInfo.cpu_usage <=| config.session.system_info.cpu_usage);
        bind!(SystemInfo.fps <=| config.session.system_info.fps);
    })) {
        Err(e) => eprintln!("Failure in settings loader: {e}"),
        _ => {}
    }
}

fn unroll_layout(
    node: &Node,
    current_path: &String,
    mut current_depth: usize,
    max_depth: usize,
) -> Vec<SettingsMenuItem> {
    let mut ret = Vec::new();

    if current_depth > max_depth {
        return ret;
    }

    match node {
        Node::HiddenParameter() => {}
        Node::ReadOnlyParameter { name, ty } => ret.push(SettingsMenuItem {
            name: (*name).into(),
            param_type: (*ty).into(),
            param_path: format!("{current_path}{name}").into(),
            item_type: SettingsMenuItemType::ReadOnlyParameter,
        }),
        Node::Parameter { name, ty } => ret.push(SettingsMenuItem {
            name: (*name).into(),
            param_type: (*ty).into(),
            param_path: format!("{current_path}{name}").into(),
            item_type: SettingsMenuItemType::Parameter,
        }),
        Node::Page { name, items } => {
            let path: String = format!("{current_path}{name}");

            current_depth += 1;
            ret.push(SettingsMenuItem {
                name: (*name).into(),
                param_type: "".into(),
                param_path: (&path).into(),
                item_type: SettingsMenuItemType::Page,
            });

            let child_path = format!("{path}.");

            for item in items {
                ret.extend_from_slice(
                    unroll_layout(item, &child_path, current_depth, max_depth).as_slice(),
                );
            }
        }
    }

    ret
}

fn unroll_path<'a>(
    node: &'a Node,
    split_path: &Vec<&str>,
    current_path: String,
    current_depth: usize,
) -> (&'a Node, String) {
    if split_path.len() == current_depth {
    } else if let Node::Page { name: _, items } = node {
        for item in items {
            if let Node::Page { name, items: _ } = item {
                if let Some(current_item) = split_path.get(current_depth + 1) {
                    if current_item == name {
                        return unroll_path(
                            item,
                            split_path,
                            format!("{}.", split_path[..=(current_depth)].join(".")),
                            current_depth + 1,
                        );
                    }
                }
            }
        }
    }

    (node, current_path)
}

pub fn bind_config_layout(handle_weak: Weak<App>, config: Arc<Config>) {
    if let Some(handle) = handle_weak.upgrade() {
        let ui_layout = handle.global::<SettingsLayout>();
        let backend_layout = Rc::new(config.get_page_layout());

        {
            let backend_layout = backend_layout.clone();

            ui_layout.on_get_page_layout(move |path| {
                let path: Vec<&str> = path.split_terminator(".").collect();
                let (layout, current_path) = unroll_path(&backend_layout, &path, String::new(), 0);

                let ret = unroll_layout(layout, &current_path, 0, 1);
                ret.as_slice()[1..].into() // remove the current page as an entry
            });
        }

        ui_layout.on_get_page_parent(move |path| {
            let mut ret = String::from(path);

            if let Some((rest, _)) = String::from(&ret).rsplit_once(".") {
                ret = rest.into();
            }

            ret.into()
        });

        {
            let config = config.clone();
            let backend_layout = backend_layout.clone();

            ui_layout.on_set_by_path(move |path, value| {
                if let Node::Page { name, items: _ } = &*backend_layout {
                    if let Some(path) = path.strip_prefix(&format!("{}.", *name)) {
                        config.set_by_path(path, &value.as_str());
                    }
                }
            });
        }

        {
            let config = config.clone();
            let backend_layout = backend_layout.clone();

            ui_layout.on_get_by_path(move |path| {
                let mut ret = "error".into();

                if let Node::Page { name, items: _ } = &*backend_layout {
                    if let Some(path) = path.strip_prefix(&format!("{}.", *name)) {
                        ret = config.get_by_path(path).into();
                    }
                }

                ret
            });
        }
    }
}
