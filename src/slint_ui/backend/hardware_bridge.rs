use crate::hardware::hardware_backend::HardwareBackend;
use crate::slint_generatedApp::{App, DebugMenuState};

use slint::platform::{Key, WindowEvent};
use slint::{ComponentHandle, Weak};
use tokio::select;

use std::sync::Arc;

pub fn bridge(ui: Weak<App>, backend: Arc<HardwareBackend>) {
    if let Some(ui) = ui.upgrade() {
        let debug_menu_state = ui.global::<DebugMenuState>();

        {
            let backend = backend.clone();
            debug_menu_state.on_debug_suspend(move || backend.power_suspend());
        }

        {
            let backend = backend.clone();
            let _ = slint::spawn_local(async_compat::Compat::new(async move {
                let mut nav_forward = backend.nav_forward.watch();
                loop {
                    select! {
                        Ok(_) = nav_forward.changed() => {
                            let value = *nav_forward.borrow_and_update();
                            if value {
                                ui.window().dispatch_event(WindowEvent::KeyPressed {
                                    text: Key::Backtab.into(),
                                });
                            } else {
                                ui.window().dispatch_event(WindowEvent::KeyReleased {
                                    text: Key::Backtab.into(),
                                });
                            }
                        },
                        else => {
                            break;
                        },
                    };
                }
            }));
        }
    }
}
