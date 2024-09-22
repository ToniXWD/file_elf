use std::thread;

use backend::{file_checker, writer::SENDER};
use cache::cache::init_trie;
use config::CONF;
use db::DB;

// file_elf
pub mod backend;
pub mod cache;
pub mod config;
pub mod db;
pub mod util;

#[cfg(feature = "webserver")]
pub mod server;

pub async fn launch_elf() {
    init_trie(DB.clone());

    let mut handlers = Vec::new();
    for target in &CONF.database.targets {
        let db_sender = SENDER.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            file_checker(target, db_sender);
        });
        handlers.push(handle);
    }

    // 启动 Rocket 服务器
    #[cfg(feature = "webserver")]
    {
        use server::init_route;
        init_route().await;
    }

    for hanlder in handlers {
        hanlder.join().unwrap();
    }
}
