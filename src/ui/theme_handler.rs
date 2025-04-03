use slint::{ComponentHandle, Weak};

use crate::slint_generatedApp::*;

pub fn handle_theme(ui: Weak<App>) {
    if let Some(ui) = ui.upgrade() {
        // let ui = &ui;
        let themes = ui.global::<Themes>();
        themes.on_theme_changed(|new_theme| {
            // TODO: see src/slint-ui/themes/themes.slint
            // println!("User requested theme change to: \"{new_theme}\"")
        });
    }
}
