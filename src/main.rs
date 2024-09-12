use std::thread;

use file_elf::{
    backend::file_checker, cache::cache::init_trie, config::CONF, db::DB, server::init_route,
};

#[rocket::main]
async fn main() {
    init_trie(DB.clone());

    let mut handlers = Vec::new();
    for target in &CONF.database.targets {
        let new_db = DB.clone();
        let handle = thread::spawn(move || {
            file_checker(new_db, target);
        });
        handlers.push(handle);
    }

    // 启动 Rocket 服务器
    init_route().await;

    for hanlder in handlers {
        hanlder.join().unwrap();
    }
}
