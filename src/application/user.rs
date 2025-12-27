use crate::data::units::UnitSystem;
use crate::parameter_struct;

use tokio::sync::watch;
use tokio::time::{Duration, timeout};
use toml;

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

parameter_struct! {ConfigManager {
    loaded: bool,

    user {
        theme {
            pub selected_theme: String = String::from("Default"),
        },
        general {
            pub unit_system: UnitSystem,
            pub disable_hill_assist: bool = false,
        },
        accessibility {
            pub animations_enabled: bool = true,
            pub accessible_switches: bool = false,
        },
    },

    session {
        debug_session {
            pub debug_mode: bool = cfg!(debug_assertions),
            pub debug_highlights: bool = false,
            pub debug_overlay_enabled: bool = true,
        },

        debug_hardware_backend_data {
            pub adc_val: i32,
        },

        simulation {
            pub running_simulation: bool = true,
        },

        can {
            pub running_can: bool = true,
        },

        system_info {
            pub total_memory_mb: i32,
            pub used_memory_mb: i32,
            pub process_memory_mb: i32,
            pub process_memory_max_mb: i32,
            pub num_cpus: i32,
            pub cpu_usage: f32,
            pub fps: i32,
        },
    },
}}

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
