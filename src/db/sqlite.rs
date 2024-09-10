use super::*;
use meta::EntryMeta;
use rusqlite::{params, Connection, NO_PARAMS};

/// 定义一个具体的 SQLite 数据库实现
pub struct SqliteDatabase {
    conn: Connection,
}

impl SqliteDatabase {
    /// 创建一个新的 `SqliteDatabase` 实例
    pub fn new(database_path: &str) -> Result<Self, CustomError> {
        let conn = if std::path::Path::new(database_path).exists() {
            Connection::open(database_path)?
        } else {
            Connection::open(database_path)?
        };

        let res = SqliteDatabase { conn };
        res.create_table()?;
        Ok(res)
    }
}

impl Database for SqliteDatabase {
    fn create_table(&self) -> Result<(), CustomError> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS access_records (
                entry TEXT PRIMARY KEY NOT NULL,
                meta BLOB
            )",
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn create_event(&self, path: &PathBuf) -> Result<(), CustomError> {
        let entry_name = path.file_name().unwrap().to_str().unwrap();
        let entry = EntryMeta::new(path)?;
        let entry_s = bincode::serialize(&entry).unwrap();
        self.conn.execute(
            "INSERT INTO access_records (entry, meta) VALUES (?1, ?2)",
            params![entry_name, entry_s],
        )?;
        Ok(())
    }

    /// 按主键查找元数据
    fn find_by_entry(&self, entry: &str) -> Result<Option<EntryMeta>, CustomError> {
        let mut stmt = self
            .conn
            .prepare("SELECT meta FROM access_records WHERE entry = ?1")?;
        let mut rows = stmt.query(params![entry])?;

        if let Some(row) = rows.next()? {
            let meta: Vec<u8> = row.get(0)?;
            let entry_meta: EntryMeta =
                bincode::deserialize(&meta).map_err(|e| CustomError::Bincode(*e))?;
            Ok(Some(entry_meta))
        } else {
            Ok(None)
        }
    }

    /// 按主键删除记录
    fn delete_by_entry(&self, entry: &str) -> Result<(), CustomError> {
        self.conn.execute(
            "DELETE FROM access_records WHERE entry = ?1",
            params![entry],
        )?;
        Ok(())
    }

    /// 按主键更新 meta
    fn update_meta(&self, entry: &str, meta: EntryMeta) -> Result<(), CustomError> {
        let meta_s = bincode::serialize(&meta).map_err(|e| CustomError::Bincode(*e))?;
        self.conn.execute(
            "UPDATE access_records SET meta = ?1 WHERE entry = ?2",
            params![meta_s, entry],
        )?;
        Ok(())
    }
}
