use crate::data::parameters::Parameter;
use crate::data::units::UnitSystem;

use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio::time::{Duration, timeout};
use toml;

use std::any::Any;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

const CONFIG_NAME: &str = "user_config.toml";
const LOAD_TIMEOUT_SECS: u64 = 10;

#[derive(Debug)]
pub enum SaveError {
    DirError, // TODO: make proper errors
    Error(Box<dyn std::error::Error>),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirError => write!(f, "Directory error"),
            Self::Error(e) => write!(f, "Failed to save: {e:?}"),
        }
    }
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

macro_rules! data_root {
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

parameter_struct! {pub ThemeConfig {
    selected_theme: String = String::from("Default"),
}}

parameter_struct! {pub GeneralConfig {
    unit_system: UnitSystem,
    disable_hill_assist: bool = false,
}}

parameter_struct! {pub AccessibilityConfig {
    animations_enabled: bool = true,
    accessible_switches: bool = false,
}}

parameter_struct! {pub DebugHardwareBackendData {
    adc_val: i32
}}

parameter_struct! {pub DebugSessionConfig {
    debug_highlights: bool = false,
    debug_overlay_enabled: bool = true
}}

parameter_struct! {pub CanConfig {
    running_can: bool = true,
}}

parameter_struct! {pub SimulationConfig {
    simulation_running: bool = true,
}}

parameter_struct! {pub StaticCarData {
    vin: String,
    odometer: u32,
}}

parameter_struct! {pub SystemInfo {
    total_memory_mb: i32,
    used_memory_mb: i32,
    process_memory_mb: i32,
    process_memory_max_mb: i32,
    num_cpus: i32,
    cpu_usage: f32,
    fps: i32
}}

data_root! {pub SessionData {
    debug_session: DebugSessionConfig,
    debug_hardware_backend_data: DebugHardwareBackendData,
    simulation: SimulationConfig,
    can: CanConfig,
    system_info: SystemInfo,
}}

data_root! {pub UserConfig {
    theme: ThemeConfig,
    general: GeneralConfig,
    accessibility: AccessibilityConfig,
}}

#[derive(Default)]
pub struct ConfigManager {
    loaded: Parameter<bool>,
    pub user: UserConfig,
    pub session: SessionData,
}

impl ConfigManager {
    pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let exe_dir = Some(env::current_exe()?.to_path_buf()).unwrap();

        let config_dir = exe_dir.parent().unwrap().join(format!(
            "{}-config/",
            exe_dir.file_name().unwrap().display()
        ));
        match fs::create_dir_all(&config_dir) {
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => (),
            Err(e) => return Err(e.into()),
            _ => (),
        }

        Ok(config_dir)
    }

    pub fn get_user_config_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;

        let user_config_dir = config_dir.join(CONFIG_NAME);
        match File::create_new(&user_config_dir) {
            Ok(mut file) => {
                let toml_string = toml::to_string_pretty(&self.user)?;
                file.write(toml_string.as_bytes())?;
            }
            _ => {}
        };

        Ok(user_config_dir)
    }

    pub async fn load_from_fs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_config_dir = self.get_user_config_path()?;
        let user_config_file = fs::read_to_string(&user_config_dir)?;

        if let Err(_) = timeout(Duration::from_secs(LOAD_TIMEOUT_SECS), async move {
            if let Ok(loaded_user_config) = toml::from_str(user_config_file.as_str()) {
                self.user.apply(loaded_user_config);
                self.loaded.set_value(true);
            }
        })
        .await
        {}

        Ok(())
    }

    pub fn save_to_fs(&self) -> Result<(), SaveError> {
        let toml_str =
            toml::to_string_pretty(&self.user).map_err(|e| SaveError::Error(e.into()))?;

        let user_config_path = self
            .get_user_config_path()
            .map_err(|e| SaveError::Error(e.into()))?;
        let dir = user_config_path.parent().ok_or(SaveError::DirError)?;
        let temp_dir = dir.join(format!("{CONFIG_NAME}.tmp"));

        let mut temp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_dir)
            .map_err(|e| SaveError::Error(e.into()))?;

        temp_file
            .write_all(toml_str.as_bytes())
            .map_err(|e| SaveError::Error(e.into()))?;

        temp_file
            .sync_all()
            .map_err(|e| SaveError::Error(e.into()))?;

        fs::rename(&temp_dir, &user_config_path).map_err(|e| SaveError::Error(e.into()))?;

        OpenOptions::new()
            .read(true)
            .open(dir)
            .map_err(|e| SaveError::Error(e.into()))?
            .sync_all()
            .map_err(|e| SaveError::Error(e.into()))?;

        Ok(())
    }

    #[allow(unused)]
    pub fn loaded(&self) -> watch::Receiver<bool> {
        self.loaded.watch()
    }
}
