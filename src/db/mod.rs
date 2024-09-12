pub mod meta;
pub mod sqlite;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
pub use meta::EntryMeta;
use rusqlite::Error as RusqliteError;

lazy_static! {
    pub static ref DB: Arc<Mutex<dyn Database>> = Arc::new(Mutex::new(
        SqliteDatabase::new(&CONF.database.path).unwrap(),
    ));
}

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
pub trait Database: Send {
    fn get_db_path(&self) -> &PathBuf;
    fn find_all(&self) -> Vec<(String, EntryMeta)>;
    fn create_table(&self) -> Result<(), CustomError>;
    fn insert_rec(&self, path: &PathBuf, meta: &EntryMeta) -> Result<(), CustomError>;
    fn find_by_entry(&self, entry: &str) -> Result<Vec<EntryMeta>, CustomError>;
    fn find_by_path(&self, path: &PathBuf) -> Result<Option<EntryMeta>, CustomError>;
    fn delete_by_entry(&self, entry: &str) -> Result<(), CustomError>;
    fn delete_by_path(&self, entry: &PathBuf) -> Result<(), CustomError>;
    fn update_meta(&self, path: &PathBuf, meta: &EntryMeta) -> Result<(), CustomError>;
    fn delete_all(&self) -> Result<(), CustomError>;
}

use crate::config::CONF;

// 导入具体的数据库实现
pub use self::sqlite::SqliteDatabase;
