use crate::data::parameters::FieldParameter;
use crate::data::units::UnitSystem;

use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::sync::watch;
use toml;

macro_rules! default_value {
    ($param_default:expr) => {
        $param_default.into()
    };
    () => {
        Default::default()
    };
}

macro_rules! parameter_struct {
    ($struct_visibillity:vis $struct_name:ident {$($param_name:tt: $param_type:ty $(= $param_default:expr)?),+ $(,)?}) => {
        #[derive(Serialize, Deserialize)]
        $struct_visibillity struct $struct_name {
            $(
                #[serde(default)]
                pub $param_name: FieldParameter<$param_type>,
            )*
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $(
                        $param_name: default_value!($($param_default)?)
                    ),*
                }
            }
        }
    };
}

parameter_struct! {pub ThemeSettings {
    selected_theme: String = String::from("Default"),
}}

parameter_struct! {pub GeneralSettings {
    unit_system: UnitSystem,
    disable_hill_assist: bool = false,
}}

parameter_struct! {pub AccessibilitySettings {
    animations_enabled: bool = true,
    accessible_switches: bool = false,
}}

parameter_struct! {pub DebugSessionSettings {
    debug_highlights: bool = true,
    debug_overlay_enabled: bool = true,
}}

parameter_struct! {pub SimulationSettings {
    simulation_running: bool = true,
}}

parameter_struct! {pub StaticCarData {
    vin: String,
    odometer: u32,
}}

#[derive(Default)]
pub struct SessionSettings {
    pub debug_session_settings: DebugSessionSettings,
    pub simulation_settings: SimulationSettings,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserSettings {
    #[serde(default)]
    pub theme: ThemeSettings,
    #[serde(default)]
    pub general: GeneralSettings,
    #[serde(default)]
    pub accessibility: AccessibilitySettings,
    #[serde(default)]
    pub static_car_data: StaticCarData,
}

#[derive(Default)]
pub enum SaveStatus {
    #[default]
    Sucess,
    Failed(Box<dyn std::error::Error>),
}

#[derive(Default)]
pub struct SettingsManager {
    loaded: watch::Sender<bool>,
    pub user_settings: UserSettings,
    pub session_settings: SessionSettings,
    static_car_data: StaticCarData,
}

impl SettingsManager {
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
        let user_settings_dir = self.get_user_settings_dir()?;

        let user_settings_file = fs::read_to_string(&user_settings_dir)?;
        self.user_settings = toml::from_str(user_settings_file.as_str())?;

        self.loaded.send_replace(true);
        Ok(())
    }

    pub fn save_to_fs(&self) -> Result<SaveStatus, Box<dyn std::error::Error>> {
        let user_settings_dir = self.get_user_settings_dir()?;

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
