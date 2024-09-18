use std::{sync::mpsc, thread};

use file_elf::{
    backend::{file_checker, writer},
    cache::cache::init_trie,
    config::CONF,
    db::DB,
    server::init_route,
};

#[rocket::main]
async fn main() {
    init_trie(DB.clone());

    let (sender, receiver) = mpsc::channel();

    let mut handlers = Vec::new();
    for target in &CONF.database.targets {
        let db_sender = sender.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            file_checker(target, db_sender);
        });
        handlers.push(handle);
    }

    drop(sender);

    // 启动后台数据库写入线程
    std::thread::spawn(move || {
        writer::db_writer(DB.clone(), receiver);
    });

    // 启动 Rocket 服务器
    init_route().await;

    for hanlder in handlers {
        hanlder.join().unwrap();
    }
}
