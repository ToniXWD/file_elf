use super::*;
use rusqlite::{Connection, Result, NO_PARAMS};

/// 定义一个具体的 SQLite 数据库实现
pub struct SqliteDatabase {
    conn: Connection,
}

impl SqliteDatabase {
    /// 创建一个新的 `SqliteDatabase` 实例
    pub fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::create_table(&conn)?;
        Ok(SqliteDatabase { conn })
    }

    fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS access_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            NO_PARAMS,
        )?;
        Ok(())
    }
}

impl Database for SqliteDatabase {
    type Conn = Connection;

    fn init_db() -> Result<Self::Conn> {
        SqliteDatabase::new().map(|db| db.conn)
    }

    fn insert_event(&self, path: &str) -> Result<()> {
        self.conn.execute("INSERT INTO access_records (path) VALUES (?)", &[&path])
            .map(|_| ())
    }
}