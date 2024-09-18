mod util;
pub mod writer;

use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::{sync::mpsc, time::Duration};

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use writer::DbAction;

use crate::config::CONF;
use crate::db::DB;

use crate::cache::CACHER;
use crate::util::is_blacklisted; // 导入缓存模块

pub fn file_checker(target: &str, db_sender: mpsc::Sender<DbAction>) {
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    // 监听指定目录
    watcher
        .watch(target, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("Watching directory for changes...");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                    new_event_handler(&path, &db_sender);
                }
                DebouncedEvent::Remove(path) => {
                    del_event_handler(&path, &db_sender);
                }
                DebouncedEvent::Rename(old_path, new_path) => {
                    println!("File renamed from {:?} to {:?}", old_path, new_path);
                    del_event_handler(&old_path, &db_sender);
                    new_event_handler(&new_path, &db_sender);
                }
                DebouncedEvent::NoticeWrite(path) => {
                    let db_guard = DB.lock().unwrap();
                    let db_path = db_guard.get_db_path().clone();
                    drop(db_guard);
                    if path.eq(&db_path) {
                        println!("Database file changed, ignoring...");
                        continue;
                    }
                    println!("File notice write: {:?}", path);
                }
                DebouncedEvent::NoticeRemove(path) => {
                    println!("File notice remove: {:?}", path);
                }
                DebouncedEvent::Chmod(path) => {
                    println!("File permissions changed: {:?}", path);
                }
                DebouncedEvent::Rescan => {
                    println!("Directory rescan occurred.");
                }
                DebouncedEvent::Error(_, err) => {
                    println!("An error occurred: {:?}", err);
                }
            },
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}

fn new_event_handler(path: &PathBuf, db_sender: &Sender<DbAction>) {
    let mut cacher_guard = match CACHER.lock() {
        Ok(guard) => guard,
        Err(e) => panic!("lock cache error: {}", e),
    };

    if path.eq(&CONF.database.path) {
        println!("Database file changed, ignoring...");
        return;
    }

    if is_blacklisted(path) {
        println!("File: {:#?} is blacklisted, ignoring...", path);
        return;
    }
    // 判断path是文件夹还是文件
    println!("new_event_handler: File created or wirte: {:?}", path);

    if let Some(meta) = cacher_guard.search_path(&path, true) {
        // 缓存中存在, 更新数据库
        match db_sender.send(DbAction::UPDATE(path.clone(), meta)) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("send update DbAction error: {}", e);
            }
        }
    } else {
        // 缓存中不存在
        // 缓存中没有, 尝试从数据库中获取

        // 先drop cache锁避免死锁
        drop(cacher_guard);

        let mut update_data = None;
        let mut need_create = true;

        match DB.lock() {
            Ok(db_guard) => {
                if let Ok(meta) = db_guard.find_by_path(&path) {
                    // 数据库查询到后还需要更新到缓存
                    update_data = meta;
                    // 数据库中存在, 不需要更新数据库
                    need_create = false;
                }
            }
            Err(e) => {
                eprintln!("lock db error: {}", e);
            }
        }

        let mut cacher_guard = match CACHER.lock() {
            Ok(guard) => guard,
            Err(e) => panic!("lock cache error: {}", e),
        };

        match cacher_guard.add_path(&path, update_data.clone(), true) {
            Ok(meta) => {
                update_data = meta;
            }
            Err(e) => {
                eprintln!("add path error: {}", e);
            }
        }

        drop(cacher_guard); // 提前释放cache锁

        if need_create {
            // 如果数据库没有查到数据, 则是新增数据, 需要插入数据库
            match db_sender.send(DbAction::CREATE(path.clone(), update_data.unwrap())) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("send create DbAction error: {}", e)
                }
            }
        }
    }
}

#[allow(unused)]
fn del_event_handler(path: &PathBuf, db_sender: &Sender<DbAction>) {
    // TODO: 后续需要支持在配置文件里面设置黑名单
    if path.eq(&CONF.database.path) {
        println!("Database file changed, ignoring...");
        return;
    }

    if is_blacklisted(path) {
        println!("File: {:#?} is blacklisted, ignoring...", path);
        return;
    }

    println!("del_event_handler: File removed: {:?}", path);

    let mut cacher_guard = CACHER.lock().unwrap();

    cacher_guard.remove_path(&path); // 缓存删除

    drop(cacher_guard); // 提前释放cache 锁避免死锁

    match db_sender.send(DbAction::DELETE(path.clone())) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("send del DbAction error: {}", e);
        }
    }

    println!("del_event_handler: send del: {:#?} DbAction success", &path)
}
