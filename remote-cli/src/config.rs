use std::fs;

use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: Connection,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub host: String,
    pub pin: u32,
}

pub fn read_config() -> Result<Config> {
    //let config_dir = option_env!("XDG_CONFIG_DIR").unwrap_or(default);
    let mut config_file_path =
        dirs::config_dir().ok_or_else(|| Error::NoConfig(String::from("No config dir found")))?;
    //.ok_or(Error::NoConfig(String::from("No config dir found")))?;

    config_file_path.push("radio");
    config_file_path.push("config.toml");

    let toml_str = fs::read_to_string(&config_file_path)
        .map_err(|_| Error::NoConfig(format!("{} does not exist", config_file_path.display())))?;
    toml::from_str(&toml_str).map_err(|err| Error::InvalidConfig(err.to_string()))
}
