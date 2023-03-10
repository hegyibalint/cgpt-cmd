use std::fmt::{Display, Formatter};
use std::fs;
use std::io::ErrorKind;
use std::path::{PathBuf};
use std::io::{Error};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub api_key: String,
    pub model: String,
}

#[derive(Debug)]
pub enum LoadingError {
    IOError(Error),
    ConfigNotFound(Error),
    TomlParsingError(toml::de::Error),
}

impl Display for LoadingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALALALALLALALAH")
    }
}

pub fn load_config() -> Result<Config, LoadingError> {
    let config_path = match get_config_file_path() {
        Ok(path) => path,
        Err(err) => return Err(LoadingError::IOError(err)),
    };

    let config_contents = match fs::read_to_string(config_path) {
        Ok(contents) => contents,
        Err(err) => return Err(LoadingError::ConfigNotFound(err))
    };

    return match toml::from_str(&config_contents) {
        Ok(data) => Ok(data),
        Err(err) => Err(LoadingError::TomlParsingError(err))
    };
}

fn get_config_file_path() -> Result<PathBuf, Error> {
    let config_dir = get_config_dir_path()?;
    return Ok(config_dir.join("config.toml"));
}

fn get_config_dir_path() -> Result<PathBuf, Error> {
    match dirs::config_dir() {
        Some(config_dir) => Ok(config_dir.join("cgpt")),
        None => Err(Error::new(ErrorKind::NotFound, "Config directory cannot be determined"))
    }
}