use slint::Weak;

use crate::application::serdes::{SerdesManager, UserSettings};
use crate::slint_generatedApp::App;

pub fn bridge_settings(app: Weak<App>, serdes_manager: SerdesManager) {
    match slint::spawn_local(async_compat::Compat::new(async move {
        match serdes_manager.loaded().wait_for(|loaded| *loaded).await {
            Err(e) => eprintln!("Failed to wait for serdes manager loading: {e}"),
            _ => {}
        }

        // TODO: implement
    })) {
        Err(e) => eprintln!("Failure in settings loader"),
        _ => {}
    }
}
