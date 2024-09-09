mod db;
mod server;

use db::SqliteDatabase;
use server::file_checker;
fn main() {
    let db: SqliteDatabase = SqliteDatabase::new().unwrap();

    file_checker(&db);
}
