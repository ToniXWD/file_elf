use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

use crate::util::errors::CustomError;

lazy_static! {
    pub static ref CONF: Config = load_config("base.toml").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub dbtype: String,
    pub path: String,
    pub targets: Vec<String>,
}

pub fn load_config(path: &str) -> Result<Config, CustomError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
