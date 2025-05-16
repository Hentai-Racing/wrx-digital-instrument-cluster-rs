use crate::{App, GlobalThemeData};
use slint::{ComponentHandle, Weak};

pub fn handle_theme(ui: Weak<App>) {
    if let Some(ui) = ui.upgrade() {
        let themes = ui.global::<GlobalThemeData>();
        themes.on_theme_changed(|_new_theme| {
            // TODO: see src/slint-ui/themes/themes.slint
            // println!("User requested theme change to: \"{new_theme}\"")
        });
    }
}
