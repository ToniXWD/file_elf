use super::*;
use meta::EntryMeta;
use rusqlite::{params, Connection};

/// 定义一个具体的 SQLite 数据库实现
pub struct SqliteDatabase {
    conn: Connection,
    db_path: PathBuf,
}

impl SqliteDatabase {
    /// 创建一个新的 `SqliteDatabase` 实例
    pub fn new(database_path: &str) -> Result<Self, CustomError> {
        let conn = if std::path::Path::new(database_path).exists() {
            Connection::open(database_path)?
        } else {
            Connection::open(database_path)?
        };

        let res = SqliteDatabase {
            conn,
            db_path: PathBuf::from(database_path),
        };
        res.create_table()?;
        Ok(res)
    }
}

impl Database for SqliteDatabase {
    fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }

    fn create_table(&self) -> Result<(), CustomError> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS access_records (
                entry TEXT PRIMARY KEY NOT NULL,
                meta BLOB
            )",
            params![],
        )?;
        Ok(())
    }

    /// 查询所有记录的迭代器
    fn find_all(&self) -> Vec<(String, EntryMeta)> {
        let mut stmt = self
            .conn
            .prepare("SELECT entry,meta FROM access_records")
            .unwrap();
        let mut rows = stmt.query(params![]).unwrap();
        let mut recs: Vec<(String, EntryMeta)> = vec![];
        // TODO: 目前返回全量数据, 应该优化成迭代器
        while let Some(row) = rows.next().unwrap() {
            let entry: String = row.get(0).unwrap();
            let meta: Vec<u8> = row.get(1).unwrap();
            let entry_meta: EntryMeta = bincode::deserialize(&meta)
                .map_err(|e| CustomError::Bincode(*e))
                .unwrap();
            recs.push((entry, entry_meta));
        }
        recs
    }

    fn create_event(&self, path: PathBuf, meta: EntryMeta) -> Result<(), CustomError> {
        let entry_name = path.file_name().unwrap().to_str().unwrap();
        let entry_s = bincode::serialize(&meta).unwrap();
        self.conn.execute(
            "INSERT INTO access_records (entry, meta) VALUES (?1, ?2)",
            params![entry_name, entry_s],
        )?;
        Ok(())
    }

    /// 按主键查找元数据
    fn find_by_entry(&self, entry: String) -> Result<Option<EntryMeta>, CustomError> {
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
    fn delete_by_entry(&self, entry: String) -> Result<(), CustomError> {
        self.conn.execute(
            "DELETE FROM access_records WHERE entry = ?1",
            params![entry],
        )?;
        Ok(())
    }

    /// 按主键更新 meta
    fn update_meta(&self, entry: String, meta: EntryMeta) -> Result<(), CustomError> {
        // 如果entry为主键的记录不存在, 则插入
        if self.find_by_entry(entry.clone())?.is_none() {
            self.create_event(meta.path.clone(), meta)?;
        } else {
            let meta_s = bincode::serialize(&meta).map_err(|e| CustomError::Bincode(*e))?;
            self.conn.execute(
                "UPDATE access_records SET meta = ?1 WHERE entry = ?2",
                params![meta_s, entry],
            )?;
        }

        Ok(())
    }
}

mod test {
    #[test]
    fn test_find_all() {
        use super::*;
        let db = SqliteDatabase::new("/home/toni/proj/file_elf/sqlite3.db").unwrap();

        let res = db.find_all();
        println!("{:?}", res);
    }
}
