use std::{
    sync::{Arc, Mutex},
    thread,
};

use file_elf::{
    cache::cache::init_trie,
    config,
    db::{Database, SqliteDatabase},
    server::file_checker,
};

fn main() {
    let conf = config::load_config("base.toml").unwrap();
    let db: Arc<Mutex<dyn Database>> = Arc::new(Mutex::new(
        SqliteDatabase::new(&conf.database.path).unwrap(),
    ));

    init_trie(db.clone());

    let mut handlers = Vec::new();
    for target in conf.database.targets {
        let new_db = db.clone();
        let handle = thread::spawn(move || {
            file_checker(new_db, target);
        });
        handlers.push(handle);
    }

    for hanlder in handlers {
        hanlder.join().unwrap();
    }
}
