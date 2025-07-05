use crate::data::parameters::FieldParameter;
use crate::data::units::UnitSystem;
use crate::slint_generatedApp::App;

use serde::{Deserialize, Serialize};
use slint::Weak;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::sync::watch;
use toml;

#[derive(Serialize, Deserialize)]
pub struct ThemeSettings {
    pub selected_theme: FieldParameter<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GeneralSettings {
    pub unit_system: FieldParameter<UnitSystem>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserSettings {
    pub theme: ThemeSettings,
    pub general: GeneralSettings,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            // TODO: change themes to enum and default there
            selected_theme: String::from("Default").into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct StaticCarData {
    vin: FieldParameter<String>,
    odometer: FieldParameter<u32>,
}

#[derive(Default)]
pub enum SaveStatus {
    #[default]
    Sucess,
    Failed(Box<dyn std::error::Error>),
}

#[derive(Default)]
pub struct SerdesManager {
    loaded: watch::Sender<bool>,
    pub user_settings: UserSettings,
    static_car_data: StaticCarData,
    app: Weak<App>,
}

impl SerdesManager {
    pub fn new(app: Weak<App>) -> Self {
        Self {
            app,
            ..Default::default()
        }
    }

    pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let exe_dir = Some(env::current_exe()?.to_path_buf()).unwrap();

        let config_dir = exe_dir.parent().unwrap().join(format!(
            "{}-config/",
            exe_dir.file_name().unwrap().display()
        ));
        match fs::create_dir(&config_dir) {
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => (),
            Err(e) => return Err(e.into()),
            _ => (),
        }

        Ok(config_dir)
    }

    pub fn get_user_settings_dir(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;

        let user_settings_dir = config_dir.join("user_settings.toml");
        match File::create_new(&user_settings_dir) {
            Ok(mut file) => {
                let toml_string = toml::to_string_pretty(&self.user_settings)?;
                file.write(toml_string.as_bytes())?;
            }
            _ => {}
        };

        Ok(user_settings_dir)
    }

    pub fn load_from_fs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let user_settings_dir = Self::get_user_settings_dir(&self)?;

        let user_settings_file = fs::read_to_string(&user_settings_dir)?;
        self.user_settings = toml::from_str(user_settings_file.as_str())?;

        self.loaded.send_replace(true);
        Ok(())
    }

    pub fn save_to_fs(&self) -> Result<SaveStatus, Box<dyn std::error::Error>> {
        let user_settings_dir = Self::get_user_settings_dir(&self)?;

        let save_status = match File::options()
            .write(true)
            .truncate(true)
            .open(&user_settings_dir)
        {
            Ok(mut file) => {
                let toml_string = toml::to_string_pretty(&self.user_settings)?;
                file.write(toml_string.as_bytes())?;
                SaveStatus::Sucess
            }
            Err(e) => {
                eprintln!(
                    "Failed to open {} as write: {e}",
                    user_settings_dir.display()
                );
                SaveStatus::Failed(e.into())
            }
        };

        Ok(save_status)
    }

    pub fn loaded(&self) -> watch::Receiver<bool> {
        self.loaded.subscribe()
    }
}
