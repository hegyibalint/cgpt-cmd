use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::io::{Error};

struct Config {
    api_key: String
}

pub fn load_config(path: &Path) -> Config {

}

pub fn get_config_file_path() -> Result<PathBuf, Error> {
    let config_dir = get_config_dir_path()?;
    return Ok(config_dir.join("config.toml"));
}

fn get_config_dir_path() -> Result<PathBuf, Error> {
    match dirs::config_dir() {
        Some(config_dir) => Ok(config_dir.join("cgpt")),
        None => Err(Error::new(ErrorKind::NotFound, "Config directory cannot be determined"))
    }
}

fn create_config_dir() -> Result<(), Error> {
    let config_dir = get_config_dir_path()?;
    return fs::create_dir(config_dir);
}