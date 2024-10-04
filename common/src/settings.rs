use std::env::{self, current_dir};

use config::{Config, ConfigError, File};
use serde::de::DeserializeOwned;

pub fn load_settings() -> Result<Config, ConfigError> {
    let environment = env::var("ENV").unwrap_or_else(|_| "development".into());
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "../settings".into());

    Config::builder()
        // The lowest priority goes to the default settings which are recommended by the maintainer
        .add_source(File::with_name(&format!("{}/default", config_path)))
        // The config values which are dependent on the deployment shall be configured here.
        // These values would override the default values
        .add_source(File::with_name(&format!("{}/{}", config_path, environment)).required(false))
        // The config values which are secrets shall be placed in local.toml file.
        // This file would not be tracked by git and should me made sure they are not pushed to git
        .add_source(File::with_name(&format!("{}/local", config_path)).required(false))
        .build()
}

pub trait LoadSettings: DeserializeOwned {
    fn load() -> Self {
        let settings = load_settings().unwrap();
        settings.try_deserialize().unwrap()
    }
}
