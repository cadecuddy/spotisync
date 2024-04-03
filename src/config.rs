use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub username: String,
    pub client_id: String,
    pub client_secret: String,
}

#[cfg(target_family = "unix")]
pub fn get_config_directory() -> PathBuf {
    match env::var("XDG_CONFIG_HOME") {
        Ok(var) => Path::new(&var).join("spotisnatch").to_path_buf(),
        Err(_) => Path::new(&env::var("HOME").unwrap()).join(".config/spotisnatch"),
    }
}

impl Config {
    pub fn load_config() -> Result<Config, ConfigError> {
        let config_directory_path = get_config_directory();
        let config_file_path = config_directory_path.join("config.json");

        // attempt to read config file
        let config_file_string = read_to_string(config_file_path)?;
        // deserialize config file
        let config: Config = serde_json::from_str(&config_file_string)?;

        Ok(config)
    }

    pub fn get_username_from_config() -> Result<String, ConfigError> {
        let config = Config::load_config()?;
        Ok(config.username)
    }
}
