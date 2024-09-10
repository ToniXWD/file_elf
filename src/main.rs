use std::thread;

use file_elf::{config, db::SqliteDatabase, server::file_checker};

fn main() {
    let conf = config::load_config("/home/toni/proj/file_elf/base.toml").unwrap();
    let db: SqliteDatabase = SqliteDatabase::new(&conf.database.path).unwrap();

    // 在单独的线程中运行 file_checker
    _ = thread::spawn(move || {
        file_checker(&db);
    })
    .join()
    .unwrap();
}
