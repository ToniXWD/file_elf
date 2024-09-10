pub mod meta;
pub mod sqlite;

use std::path::PathBuf;

use meta::EntryMeta;
use rusqlite::Error as RusqliteError;

/// 定义自定义错误类型
#[derive(Debug)]
pub enum CustomError {
    Io(std::io::Error),
    Bincode(bincode::ErrorKind),
    Rusqlite(RusqliteError),
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

// 定义一个数据库操作的 trait
pub trait Database {
    fn create_table(&self) -> Result<(), CustomError>;
    fn create_event(&self, path: &PathBuf) -> Result<(), CustomError>;
    fn find_by_entry(&self, entry: &str) -> Result<Option<EntryMeta>, CustomError>;
    fn delete_by_entry(&self, entry: &str) -> Result<(), CustomError>;
    fn update_meta(&self, entry: &str, meta: EntryMeta) -> Result<(), CustomError>;
}

// 导入具体的数据库实现
pub use self::sqlite::SqliteDatabase;
