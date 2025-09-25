use crate::data::parameters::Parameter;
use crate::data::units::UnitSystem;

use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use toml;

use std::any::Any;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

pub enum SaveStatus {
    Success,
    Failed(Box<dyn std::error::Error>),
}

macro_rules! default_value {
    ($type:ty| $param_default:expr) => {
        $param_default.into()
    };
    (String| ) => {
        String::from("Unknown").into()
    };
    ($type:ty| ) => {
        Default::default()
    };
}

macro_rules! parameter_struct {
    ($struct_visibillity:vis $struct_name:ident {$($param:ident: $param_type:ty $(= $param_default:expr)?),+ $(,)?}) => {
        #[derive(Serialize, Deserialize)]
        $struct_visibillity struct $struct_name {
            $(
                #[serde(default)]
                pub $param: Parameter<$param_type>,
            )+
        }

        impl $struct_name {
            #[allow(unused)]
            fn apply(&self, from: Self) {
                $( self.$param.set_value(from.$param.value()) );*
            }

            #[allow(unused)]
            pub fn set_by_name(&self, param_name: &str, value: &dyn Any) {
                match param_name {
                    $(stringify!($param) => {
                        if let Some(value) = value.downcast_ref::<$param_type>() {
                            self.$param.set_value(value.clone())
                        } else {
                            // TODO: return err Result
                        }
                    }),+
                    _ => {
                        // TODO: return err Result
                    }
                }
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $(
                        $param: default_value!($param_type| $($param_default)?)
                    ),+
                }
            }
        }
    };
}

macro_rules! settings_root {
    ($visible:vis $root:ident {$($param:ident: $param_ty:ty),+ $(,)?}) => {
        #[derive(Default, Serialize, Deserialize)]
        $visible struct $root {
            #[serde(default)]
            $(pub $param: $param_ty),+
        }

        impl $root {
            #[allow(unused)]
            fn apply(&self, from: Self) {
                $(self.$param.apply(from.$param));*
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
    debug_highlights: bool = false,
    debug_overlay_enabled: bool = true
}}

parameter_struct! {pub CanSettings {
    running_can: bool = true,
}}

parameter_struct! {pub SimulationSettings {
    simulation_running: bool = true,
}}

parameter_struct! {pub StaticCarData {
    vin: String,
    odometer: u32,
}}

settings_root! {pub SessionSettings {
    debug_session_settings: DebugSessionSettings,
    simulation_settings: SimulationSettings,
    can_settings: CanSettings,
}}

settings_root! {pub UserSettings {
    theme: ThemeSettings,
    general: GeneralSettings,
    accessibility: AccessibilitySettings,
}}

#[derive(Default)]
pub struct SettingsManager {
    loaded: Parameter<bool>,
    pub user_settings: UserSettings,
    pub session_settings: SessionSettings,
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

    pub fn load_from_fs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_settings_dir = self.get_user_settings_dir()?;
        let user_settings_file = fs::read_to_string(&user_settings_dir)?;
        let loaded_user_settings = toml::from_str(user_settings_file.as_str())?;

        self.user_settings.apply(loaded_user_settings);
        self.loaded.set_value(true);

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
                SaveStatus::Success
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

    #[allow(unused)]
    pub fn loaded(&self) -> watch::Receiver<bool> {
        self.loaded.watch()
    }
}
