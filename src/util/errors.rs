use std::fmt;

use rusqlite::Error as RusqliteError;
use toml::de::Error as TomlError;

/// 定义自定义错误类型
#[derive(Debug)]
pub enum CustomError {
    Io(std::io::Error),
    Bincode(bincode::ErrorKind),
    Rusqlite(RusqliteError),
    Toml(TomlError),
    ErrStr(String),
    UnknownErr,
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::Io(err)
    }
}

impl From<bincode::ErrorKind> for CustomError {
    fn from(err: bincode::ErrorKind) -> Self {
        CustomError::Bincode(err)
    }
}

impl From<RusqliteError> for CustomError {
    fn from(err: RusqliteError) -> Self {
        CustomError::Rusqlite(err)
    }
}

impl From<String> for CustomError {
    fn from(err: String) -> Self {
        CustomError::ErrStr(err)
    }
}

impl From<&str> for CustomError {
    fn from(err: &str) -> Self {
        CustomError::ErrStr(err.to_string())
    }
}

impl From<TomlError> for CustomError {
    fn from(err: TomlError) -> Self {
        CustomError::Toml(err)
    }
}

// 实现 Display 特征
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Io(io_err) => write!(f, "IO error: {}", io_err),
            CustomError::Bincode(bincode_err) => write!(f, "Bincode error: {:?}", bincode_err),
            CustomError::Rusqlite(rusqlite_err) => write!(f, "Rusqlite error: {}", rusqlite_err),
            CustomError::Toml(toml_err) => write!(f, "TOML error: {}", toml_err),
            CustomError::ErrStr(err_str) => write!(f, "Error: {}", err_str),
            CustomError::UnknownErr => write!(f, "Error: {}", "unknown error"),
        }
    }
}
