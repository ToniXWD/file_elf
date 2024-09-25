use backend::{file_checker, writer::SENDER};
use cache::cache::init_trie;
use config::CONF;
use db::DB;
use logger::setup_logger;
use tokio::spawn;

// file_elf
pub mod backend;
pub mod cache;
pub mod config;
pub mod db;
pub mod logger;
pub mod server;
pub mod util;

pub async fn launch_elf() {
    let _ = setup_logger();

    spawn(init_trie(DB.clone()));

    for target in &CONF.database.targets {
        let db_sender = SENDER.clone();
        spawn(file_checker(target, db_sender));
    }

    // 启动 Rocket 服务器
    #[cfg(feature = "webserver")]
    {
        use server::init_route;
        spawn(init_route());
    }
}
