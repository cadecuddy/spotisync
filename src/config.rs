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
        Ok(var) => Path::new(&var).join("spotisync").to_path_buf(),
        Err(_) => Path::new(&env::var("HOME").unwrap()).join(".config/spotisync"),
    }
}

impl Config {
    fn create_config() -> Result<(), ConfigError> {
        let config_directory_path = get_config_directory();
        let config_file_path = config_directory_path.join("config.json");
        // create config directory if it doesn't exist
        if !config_directory_path.exists() {
            std::fs::create_dir_all(&config_directory_path)?;
        }
        // create config file if it doesn't exist
        if !config_file_path.exists() {
            let default_config = Config {
                username: String::new(),
                client_id: String::new(),
                client_secret: String::new(),
            };
            let default_config_string = serde_json::to_string_pretty(&default_config)?;
            std::fs::write(config_file_path, default_config_string)?;
        }
        Ok(())
    }

    pub fn load_config() -> Result<Config, ConfigError> {
        let config_directory_path = get_config_directory();
        let config_file_path = config_directory_path.join("config.json");

        if !config_file_path.exists() {
            Config::create_config()?;
            println!(
                "Config file didn't already exist, please fill in config file at {:?}",
                config_file_path
            );
            std::process::exit(1);
        }

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
