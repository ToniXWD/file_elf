pub mod meta;
pub mod sqlite;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use log::info;
pub use meta::EntryMeta;

lazy_static! {
    pub static ref DB: Arc<Mutex<dyn Database>> =
        Arc::new(Mutex::new(match CONF.database.dbtype.as_str() {
            "sqlite" => {
                match SqliteDatabase::new(&CONF.database.path) {
                    Ok(db) => {
                        info!(
                            "SqliteDatabase created from {:#?} successfully",
                            &CONF.database.path
                        );
                        db
                    }
                    Err(e) => panic!(
                        "Failed to create SqliteDatabase from path: {:#?}, errorr: {}",
                        CONF.database.path, e
                    ),
                }
            }
            st => {
                panic!("Unsupported database type: {}", st)
            }
        },));
}

// 定义一个数据库操作的 trait
pub trait Database: Send + Sync {
    fn get_db_path(&self) -> &PathBuf;
    fn find_all(&self) -> Vec<(String, EntryMeta)>;
    fn create_table(&self) -> Result<(), CustomError>;
    fn insert_rec(&self, path: &PathBuf, meta: &EntryMeta) -> Result<(), CustomError>;
    fn find_by_entry(&self, entry: &str) -> Result<Vec<EntryMeta>, CustomError>;
    fn find_by_path(&self, path: &PathBuf) -> Result<Option<EntryMeta>, CustomError>;
    fn find_by_path_prefix(&self, path: &PathBuf) -> Result<Vec<EntryMeta>, CustomError>;
    fn delete_by_entry(&self, entry: &str) -> Result<(), CustomError>;
    fn delete_by_path(&self, path: &PathBuf) -> Result<(), CustomError>;
    fn delete_by_path_prefix(&self, path: &PathBuf) -> Result<(), CustomError>;
    fn update_meta(&self, path: &PathBuf, meta: &EntryMeta) -> Result<(), CustomError>;
    fn delete_all(&self) -> Result<(), CustomError>;
}

use crate::{config::CONF, util::errors::CustomError};

// 导入具体的数据库实现
pub use self::sqlite::SqliteDatabase;
