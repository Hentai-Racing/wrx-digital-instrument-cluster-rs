use crate::slint_generatedApp::{App, GlobalThemeData};

use serde::{Deserialize, Serialize};
use slint::Weak;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tokio::sync::watch;
use toml;

#[derive(Serialize, Deserialize)]
pub struct UserSettings {
    selected_theme: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            selected_theme: "Default".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct StaticCarData {
    vin: String,
    odometer: u32,
}

#[derive(Default)]
pub struct SerdesManager {
    loaded: watch::Sender<bool>,
    user_settings: UserSettings,
    static_car_data: StaticCarData,
    app: Weak<App>,
}

impl SerdesManager {
    pub fn new(app: Weak<App>) -> Self {
        let (loaded_tx, _) = watch::channel(false);
        Self {
            app,
            loaded: loaded_tx,
            ..Default::default()
        }
    }

    pub fn load_from_fs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let exe_dir = env::current_exe()?
            .parent()
            .map(|x| x.to_path_buf())
            .unwrap();

        let config_dir = exe_dir.join("config");
        match fs::create_dir(&config_dir) {
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => (),
            Err(e) => return Err(e.into()),
            _ => (),
        }

        let user_settings_dir = &config_dir.join("user_settings.toml");
        match File::create_new(&user_settings_dir) {
            Ok(mut file) => {
                let toml_string = toml::to_string_pretty(&self.user_settings)?;
                file.write(toml_string.as_bytes())?;
            }
            _ => {}
        };

        let user_settings_file = fs::read_to_string(&user_settings_dir)?;
        self.user_settings = toml::from_str(user_settings_file.as_str())?;

        println!("{}", self.user_settings.selected_theme);

        self.loaded.send_replace(true);
        Ok(())
    }

    pub fn save_to_fs(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn loaded(&self) -> watch::Receiver<bool> {
        self.loaded.subscribe()
    }
}
