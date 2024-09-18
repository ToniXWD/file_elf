use std::time::{SystemTime, UNIX_EPOCH};

use super::*;
use meta::EntryMeta;
use rusqlite::{params, Connection, Row};

/// 定义一个具体的 SQLite 数据库实现
pub struct SqliteDatabase {
    conn: Connection,
    db_path: PathBuf,
}

fn system_to_unix_ts(time: &SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn row_to_meta(row: &Row<'_>) -> EntryMeta {
    let path: String = row.get(1).unwrap();
    let size: u64 = row.get(2).unwrap();
    let modified: i64 = row.get(3).unwrap();
    let access_count: u32 = row.get(4).unwrap();
    let entry_type: String = row.get(5).unwrap();

    EntryMeta {
        path: PathBuf::from(path),
        size: size as u64,
        modified: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(modified as u64),
        access_count: access_count as u32,
        entry_type: entry_type.parse().unwrap(),
    }
}

impl SqliteDatabase {
    /// 创建一个新的 `SqliteDatabase` 实例
    pub fn new(database_path: &PathBuf) -> Result<Self, CustomError> {
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
                entry TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                size INTEGER NOT NULL,
                modified INTEGER NOT NULL,
                access_count INTEGER NOT NULL,
                entry_type TEXT NOT NULL
            )",
            params![],
        )?;
        Ok(())
    }

    /// 查询所有记录的迭代器
    fn find_all(&self) -> Vec<(String, EntryMeta)> {
        let mut stmt = self.conn.prepare("SELECT * FROM access_records").unwrap();
        let mut rows = stmt.query(params![]).unwrap();
        let mut recs: Vec<(String, EntryMeta)> = vec![];
        // TODO: 目前返回全量数据, 应该优化成迭代器
        while let Some(row) = rows.next().unwrap() {
            let entry: String = row.get(0).unwrap();

            let entry_meta = row_to_meta(row);

            recs.push((entry, entry_meta));
        }
        recs
    }

    fn insert_rec(&self, path: &PathBuf, meta: &EntryMeta) -> Result<(), CustomError> {
        let entry_name = path.file_name().unwrap().to_str().unwrap();
        let e_path = path.to_string_lossy();
        self.conn.execute(
            "INSERT INTO access_records (entry, path, size, modified, access_count, entry_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![entry_name, &e_path, &meta.size, system_to_unix_ts(&meta.modified), &meta.access_count, &meta.entry_type.to_string()],
        )?;
        Ok(())
    }

    /// 按entry查找元数据
    fn find_by_entry(&self, entry: &str) -> Result<Vec<EntryMeta>, CustomError> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM access_records WHERE entry = ?1")?;
        let mut rows = stmt.query(params![entry])?;

        let mut res = vec![];
        while let Some(row) = rows.next()? {
            let meta = row_to_meta(row);
            res.push(meta);
        }
        Ok(res)
    }

    /// 按path查找元数据
    fn find_by_path(&self, path: &PathBuf) -> Result<Option<EntryMeta>, CustomError> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM access_records WHERE path = ?1")?;
        let mut rows = stmt.query(params![path.to_string_lossy()])?;

        if let Some(row) = rows.next()? {
            let meta = row_to_meta(row);
            Ok(Some(meta))
        } else {
            Ok(None)
        }
    }

    /// 按path查找元数据
    fn find_by_path_prefix(&self, path: &PathBuf) -> Result<Vec<EntryMeta>, CustomError> {
        let path_with_wildcard = format!("{}%", path.to_string_lossy());
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM access_records WHERE path LIKE ?1")?;
        let mut rows = stmt.query(params![path_with_wildcard])?;

        let mut res = Vec::new();

        while let Some(row) = rows.next()? {
            let meta = row_to_meta(row);
            res.push(meta);
        }
        Ok(res)
    }

    /// 删除指定路径的单个记录
    fn delete_by_entry(&self, entry: &str) -> Result<(), CustomError> {
        self.conn.execute(
            "DELETE FROM access_records WHERE entry = ?1",
            params![entry],
        )?;
        Ok(())
    }

    /// 按path删除记录
    fn delete_by_path(&self, entry: &PathBuf) -> Result<(), CustomError> {
        println!("delete_by_path: {}", entry.to_string_lossy());
        self.conn.execute(
            "DELETE FROM access_records WHERE path = ?1",
            params![entry.to_string_lossy()],
        )?;
        Ok(())
    }

    /// 按path前缀匹配删除记录
    fn delete_by_path_prefix(&self, path: &PathBuf) -> Result<(), CustomError> {
        let path_with_wildcard = format!("{}%", path.to_string_lossy());
        println!("delete_by_path_prefix: {}", path_with_wildcard);
        self.conn.execute(
            "DELETE FROM access_records WHERE path LIKE ?1",
            params![path_with_wildcard],
        )?;
        Ok(())
    }

    /// 按path更新 meta
    fn update_meta(&self, path: &PathBuf, meta: &EntryMeta) -> Result<(), CustomError> {
        // 如果path的记录不存在, 则插入
        match self.find_by_path(path)? {
            Some(_) => {
                // 记录存在则更新
                self.conn.execute(
                    "UPDATE access_records SET size = ?2, modified = ?3, access_count = ?4, entry_type = ?5 WHERE path = ?1",
                    params![path.to_string_lossy(), meta.size, system_to_unix_ts(&meta.modified), &meta.access_count, &meta.entry_type.to_string()],
                )?;
                Ok(())
            }
            None => {
                // 记录不存在则新建
                self.insert_rec(path, meta)?;
                Ok(())
            }
        }
    }

    /// 删除所有数据
    fn delete_all(&self) -> Result<(), CustomError> {
        self.conn.execute("DELETE FROM access_records", params![])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_db() -> SqliteDatabase {
        let db = SqliteDatabase::new(&PathBuf::from("/home/toni/proj/file_elf/sqlite3-test.db"))
            .unwrap();
        db.delete_all().unwrap();
        db
    }

    #[test]
    fn test_create_table() {
        let db = get_db();
        assert!(db.create_table().is_ok());
    }

    #[test]
    fn test_insert_and_find_by_path() {
        let db = get_db();
        db.create_table().unwrap();

        let entry_meta = EntryMeta {
            path: PathBuf::from("/test/path"),
            size: 1024,
            modified: SystemTime::now(),
            access_count: 1,
            entry_type: "Dir".parse().unwrap(),
        };

        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();
        let result = db.find_by_path(&entry_meta.path).unwrap();

        assert!(result.is_some());
        let found_meta = result.unwrap();
        assert_eq!(found_meta.path, entry_meta.path);
        assert_eq!(found_meta.size, entry_meta.size);

        let entry_meta = EntryMeta {
            path: PathBuf::from("/test2/path"), // entry相同, path不同
            size: 1024,
            modified: SystemTime::now(),
            access_count: 1,
            entry_type: "Dir".parse().unwrap(),
        };

        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();
        let result = db.find_by_path(&entry_meta.path).unwrap();

        assert!(result.is_some());
        let found_meta = result.unwrap();
        assert_eq!(found_meta.path, entry_meta.path);
        assert_eq!(found_meta.size, entry_meta.size);
    }

    #[test]
    fn test_find_by_entry() {
        let db = get_db();
        db.create_table().unwrap();

        let entry_meta = EntryMeta {
            path: PathBuf::from("/test/path"),
            size: 1024,
            modified: SystemTime::now(),
            access_count: 1,
            entry_type: "Dir".parse().unwrap(),
        };

        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();
        let result = db.find_by_entry("path").unwrap();

        assert_eq!(result.len(), 1);
        let found_meta = &result[0];
        assert_eq!(found_meta.path, entry_meta.path);
        assert_eq!(found_meta.size, entry_meta.size);
    }

    #[test]
    fn test_update_meta() {
        let db = get_db();
        db.create_table().unwrap();

        let mut entry_meta = EntryMeta {
            path: PathBuf::from("/test/path/test_update_meta_1.txt"),
            size: 1024,
            modified: SystemTime::now(),
            access_count: 1,
            entry_type: "File".parse().unwrap(),
        };

        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();

        // Update the meta
        entry_meta.size = 2048;
        db.update_meta(&entry_meta.path, &entry_meta).unwrap();

        let result = db.find_by_path(&entry_meta.path).unwrap();
        assert!(result.is_some());
        let updated_meta = result.unwrap();
        assert_eq!(updated_meta.size, 2048);
    }

    #[test]
    fn test_delete_by_path() {
        let db = get_db();
        db.create_table().unwrap();

        let entry_meta = EntryMeta {
            path: PathBuf::from("/test/path/2.txt"),
            size: 1024,
            modified: SystemTime::now(),
            access_count: 1,
            entry_type: "File".parse().unwrap(),
        };

        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();
        db.delete_by_path(&entry_meta.path).unwrap();

        let result = db.find_by_path(&entry_meta.path).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_path_prefix() {
        let db = get_db();
        db.create_table().unwrap();

        let mut entry_meta = EntryMeta {
            path: PathBuf::from("/test/path1"),
            size: 1024,
            modified: SystemTime::now(),
            access_count: 1,
            entry_type: "Dir".parse().unwrap(),
        };

        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();

        entry_meta.path = PathBuf::from("/test/path1/path2");
        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();

        entry_meta.path = PathBuf::from("/test/path1/path2/test.txt");
        entry_meta.entry_type = "File".parse().unwrap();
        db.insert_rec(&entry_meta.path, &entry_meta).unwrap();

        let result_ins = db
            .find_by_path_prefix(&PathBuf::from("/test/path1"))
            .unwrap();
        assert_eq!(result_ins.len(), 3);
        db.delete_by_path_prefix(&PathBuf::from("/test/path1"))
            .unwrap();

        let result_del = db.find_by_path(&PathBuf::from("/test/path1")).unwrap();
        assert!(result_del.is_none());
    }
}
