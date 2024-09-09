pub mod sqlite;

use rusqlite::Result;

// 定义一个数据库操作的 trait
pub trait Database {
    type Conn;
    fn init_db() -> Result<Self::Conn>;
    fn insert_event(&self, path: &str) -> Result<()>;
}

// 导入具体的数据库实现
pub use self::sqlite::SqliteDatabase;
