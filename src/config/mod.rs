use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use toml;

use crate::util::errors::CustomError;
use crate::util::to_absolute_path;

#[cfg(target_os = "windows")]
use crate::util::get_drives;

const CONFNAME: &str = "base.toml";

// 初始化CONF在setup_logger之前, 所以直接用printfln!输出
lazy_static! {
    pub static ref CONF: Config = DatabaseConfig::load_config(CONFNAME).unwrap_or_else(|_| {
        println!("No config file found, using default config");
        let def_conf = Config::default();
        def_conf
            .database
            .save_config()
            .unwrap_or(println!("Failed to save default config to file"));
        def_conf
    });
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub dbtype: String,
    pub path: PathBuf,
    pub targets: Vec<String>,
    pub blacklist: Vec<String>, // 黑名单字段
    pub hotdirnum: usize,
    pub log_level: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let mut default_config = Self {
            dbtype: "sqlite".to_string(),
            path: to_absolute_path(&PathBuf::from("sqlite3.db")).unwrap(),
            targets: vec![],
            blacklist: vec![
                ".*build.*".to_string(),
                ".*target.*".to_string(),
                ".*[Cc]ache.*.".to_string(),
                ".*node_modules.*".to_string(),
            ],
            hotdirnum: 100,
            log_level: "info".to_string(),
        };
        #[cfg(target_os = "windows")]
        {
            let mut drivers: Vec<String> = get_drives();
            drivers[0] = "C:\\Users".to_string(); // C盘不监视整个根目录
            default_config.targets.extend(drivers);

            default_config.blacklist.extend(vec![
                ".*\\.git.*".to_string(),
                ".*\\.vscode.*".to_string(),
                ".*\\.idea.*".to_string(),
                ".*\\$RECYCLE\\.BIN.*".to_string(),
                ".*WeChat Files\\\\.*\\\\Msg.*".to_string(),
                ".*WeChat Files\\\\.*\\\\config.*".to_string(),
                ".*WeChat Files\\\\.*\\\\FileStorage\\\\MsgAttach.*".to_string(),
                ".*WeChat Files\\\\.*\\\\FileStorage\\\\CustomEmotion.*".to_string(),
                ".*QQ files.*\\\\nt_qq.*\\\\nt_db.*".to_string(),
                ".*QQ files.*\\\\nt_qq.*\\\\nt_temp.*".to_string(),
                ".*QQ files.*\\\\nt_qq.*\\\\nt_data*".to_string(),
            ]);
        }
        #[cfg(not(target_os = "windows"))]
        default_config.targets.extend(vec!["/".to_string()]); // Linux or MacOS监听更系统目录

        default_config
    }
}

impl DatabaseConfig {
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

    pub fn save_config(&self) -> Result<(), CustomError> {
        // 将 DatabaseConfig 序列化为 TOML 格式字符串
        let toml_string = toml::to_string_pretty(self).expect("Failed to serialize config");

        // 创建或打开文件
        let mut file = match File::create(CONFNAME) {
            Ok(file) => {
                println!("File: {:#?} created successfully", CONFNAME);
                file
            }
            Err(error) => {
                println!("Error creating file: {:?}", error);
                return Err(CustomError::from(error));
            }
        };

        // 将 TOML 格式的字符串写入文件
        match file.write_all(toml_string.as_bytes()) {
            Ok(_) => println!("Config saved to {:#?} successfully", CONFNAME),
            Err(error) => {
                println!("Error writing to file: {:?}", error);
                return Err(CustomError::from(error));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, CONF};

    #[test]
    fn test_show_config() {
        let config_load = &*CONF;
        println!("{:#?}", config_load);

        let config_default = Config::default();
        println!("{:#?}", config_default);
    }
}
