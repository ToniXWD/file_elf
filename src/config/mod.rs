use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml;

use crate::util::errors::CustomError;
use crate::util::to_absolute_path;

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
    pub path: PathBuf,
    pub targets: Vec<String>,
    pub blacklist: Vec<String>, // 黑名单字段
    pub hotdirnum: usize,
}

pub fn load_config(path: &str) -> Result<Config, CustomError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 解析 TOML 文件
    let mut config: Config = toml::from_str(&contents)?;

    // 将 database.path 转换为绝对路径
    config.database.path = to_absolute_path(&config.database.path)?;

    Ok(config)
}
