mod util;
mod writer;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, time::Duration};

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use writer::DbMsg;

use crate::db::Database;

use crate::cache::CACHER; // 导入缓存模块

pub fn file_checker(db: Arc<Mutex<dyn Database>>, target: String) {
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    let (db_sender, db_receiver): (Sender<DbMsg>, Receiver<DbMsg>) = mpsc::channel();

    // 启动后台数据库写入线程
    let db_w = db.clone();
    std::thread::spawn(move || {
        writer::db_writer(db_w, db_receiver);
    });

    // 监听指定目录
    watcher
        .watch(target, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("Watching directory for changes...");

    let db_guard = db.lock().unwrap();
    let db_path = db_guard.get_db_path().clone();
    drop(db_guard);

    loop {
        match rx.recv() {
            Ok(event) => {
                let mut cacher_guard = CACHER.lock().unwrap();
                match event {
                    DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                        if path.eq(&db_path) {
                            println!("Database file changed, ignoring...");
                            continue;
                        }
                        // 判断path是文件夹还是文件
                        println!("File created or wirte: {:?}", path);

                        if let Some(meta) = cacher_guard.search_path(&path, true) {
                            let entry = path.file_name().unwrap().to_str().unwrap().to_string();

                            // 缓存中存在, 更新数据库
                            db_sender
                                .send(DbMsg {
                                    action: writer::DbAction::UPDATE,
                                    entry: Some(entry),
                                    path: None,
                                    meta: Some(meta),
                                })
                                .unwrap();
                        } else {
                            // 缓存中不存在
                            let entry = path.file_name().unwrap().to_str().unwrap().to_string();
                            // 缓存中没有, 尝试从数据库中获取
                            if let Ok(meta) = db.lock().unwrap().find_by_entry(entry.clone()) {
                                // 数据库查询到后还需要更新
                                if let Ok(Some(meta)) = cacher_guard.add_path(&path, meta, true) {
                                    db_sender
                                        .send(DbMsg {
                                            action: writer::DbAction::UPDATE,
                                            entry: Some(entry),
                                            path: None,
                                            meta: Some(meta),
                                        })
                                        .unwrap();
                                }
                            } else {
                                // 数据库没有, 直接在缓存新建, 新建后插入数据库
                                if let Ok(Some(meta)) = cacher_guard.add_path(&path, None, true) {
                                    db_sender
                                        .send(DbMsg {
                                            action: writer::DbAction::CREATE,
                                            entry: None,
                                            path: Some(path.clone()),
                                            meta: Some(meta),
                                        })
                                        .unwrap();
                                }
                            }
                        }
                    }
                    DebouncedEvent::Remove(path) => {
                        if path.eq(&db_path) {
                            println!("Database file changed, ignoring...");
                            continue;
                        }

                        println!("File removed: {:?}", path);
                        _ = db.lock().unwrap().delete_by_entry(
                            path.file_name().unwrap().to_str().unwrap().to_string(),
                        ); // 数据库删除
                        cacher_guard.remove_path(&path); // 缓存删除
                    }
                    DebouncedEvent::Rename(old_path, new_path) => {
                        println!("File renamed from {:?} to {:?}", old_path, new_path);
                    }
                    DebouncedEvent::NoticeWrite(path) => {
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
                }
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}
