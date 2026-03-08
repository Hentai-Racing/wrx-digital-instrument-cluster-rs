use crate::data::parameters::Bound;
use crate::data::units::UnitSystem;
use crate::parameter_struct;
use crate::slint_generatedApp::{ClusterTheme, SettingsSpecialPages};
use crate::slint_ui::backend::lang::StrLang;

use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio::time::{Duration, timeout};
use toml;

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

const CONFIG_NAME: &str = "config.toml";
const LOAD_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum SaveError {
    DirError, // TODO: make proper errors
    Error(Box<dyn std::error::Error>),
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct PageTrigger(SettingsSpecialPages);

impl Into<SettingsSpecialPages> for PageTrigger {
    fn into(self) -> SettingsSpecialPages {
        self.0
    }
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirError => write!(f, "Directory error"),
            Self::Error(e) => write!(f, "Failed to save: {e:?}"),
        }
    }
}

parameter_struct! {Config {
    [hidden] loaded: bool,

    user {
        general {
            pub disable_hill_assist_warning: bool = false,
            pub unit_system: UnitSystem,
            pub lang: StrLang,
        },

        theme {
            pub selected_theme: ClusterTheme = ClusterTheme::Default,
        },

        accessibility {
            pub animations_enabled: bool = true,
            pub accessible_switches: bool = false,
            pub selection_box_thickness: Bound<i32> = Bound::new(2, 1..=10),
        },
    },

    developer {
        // TODO: add conditional hiding for pages and params
        debug {
            pub [ro] debug_mode: bool = cfg!(debug_assertions),
            pub debug_highlights: bool = false,
            pub debug_overlay_enabled: bool = true,
            pub extra_debug_info: bool = false,
        },

        simulation {
            pub running_simulation: bool = true,
        },

        can {
            pub running_can: bool = true,
        },

        hardware {
            pub [ro] adc_val: i32,
        },

        system_info {
            pub [ro] total_memory_mb: i32,
            pub [ro] used_memory_mb: i32,
            pub [ro] process_memory_mb: i32,
            pub [ro] process_memory_max_mb: i32,
            pub [ro] num_cpus: i32,
            pub [ro] cpu_usage: f32,
            pub [ro] fps: i32,
        },
    },

    about {
        pub [ro] version: String = get_build_version(),
        pub [ro] slint_version: String = get_slint_version(),
        pub attributions: PageTrigger = PageTrigger(SettingsSpecialPages::Attributions),
        // TODO: allow icons or similar thing to allow showing the built with slint banner
        // pub [ro] built_with_slint: Icon
    },
}}

impl Config {
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

    pub fn get_config_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;

        let config_dir = config_dir.join(CONFIG_NAME);
        match File::create_new(&config_dir) {
            Ok(mut file) => {
                let toml_string = toml::to_string_pretty(&self.user)?;
                file.write(toml_string.as_bytes())?;
            }
            _ => {}
        };

        Ok(config_dir)
    }

    pub async fn load_from_fs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = self.get_config_path()?;
        let config_file = fs::read_to_string(&config_dir)?;

        if let Err(_) = timeout(LOAD_TIMEOUT, async move {
            if let Ok(loaded_config) = toml::from_str(config_file.as_str()) {
                self.user.apply(loaded_config);
                self.loaded.set_value(true);
            }
        })
        .await
        {
            eprintln!("Timeout on config load");
        }

        Ok(())
    }

    pub fn save_to_fs(&self) -> Result<(), SaveError> {
        // TODO: add timestamp and config version to file
        let toml_str =
            toml::to_string_pretty(&self.user).map_err(|e| SaveError::Error(e.into()))?;

        let config_path = self
            .get_config_path()
            .map_err(|e| SaveError::Error(e.into()))?;
        let dir = config_path.parent().ok_or(SaveError::DirError)?;
        let temp_dir = dir.join(format!("{CONFIG_NAME}.tmp"));
        let old_dir = dir.join(format!("{CONFIG_NAME}.old"));

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

        fs::copy(&config_path, &old_dir).map_err(|e| SaveError::Error(e.into()))?;
        fs::rename(&temp_dir, &config_path).map_err(|e| SaveError::Error(e.into()))?;

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

fn get_build_version() -> String {
    let mut ret = String::from(env!("CARGO_PKG_VERSION"));
    if cfg!(debug_assertions) {
        let git_rev = PathBuf::from(env!("OUT_DIR")).join("gitrev");
        if git_rev.exists() {
            if let Ok(value) = fs::read_to_string(git_rev) {
                ret.push_str(&format!(" ({})", value.trim()));
            }
        }
    } else {
    }

    ret
}

fn get_slint_version() -> String {
    let mut ret = String::from("unknown");
    let git_rev = PathBuf::from(env!("OUT_DIR")).join("slint_version");
    if git_rev.exists() {
        if let Ok(value) = fs::read_to_string(git_rev) {
            ret = value.trim().to_string();
        }
    }

    ret
}

impl Default for PageTrigger {
    fn default() -> Self {
        Self(SettingsSpecialPages::Attributions)
    }
}

impl std::str::FromStr for PageTrigger {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}

impl std::fmt::Display for PageTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
