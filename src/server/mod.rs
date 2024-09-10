mod util;

use std::{sync::mpsc, time::Duration};

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::db::Database;

use crate::cache::CACHER; // 导入缓存模块

pub fn file_checker(db: &dyn Database) {
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    // 监听指定目录
    watcher
        .watch("/home/toni/proj", RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("Watching directory for changes...");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                    // 判断path是文件夹还是文件
                    println!("File created or wirte: {:?}", path);
                    let mut guard = CACHER.lock().unwrap();

                    if !guard.contains_path(&path) {
                        // 缓存中没有, 尝试从数据库中获取
                        if let Ok(meta) =
                            db.find_by_entry(path.file_name().unwrap().to_str().unwrap())
                        {
                            guard.add_path(&path, meta, true);
                        } else {
                            // 数据库没有, 直接在缓存新建
                            guard.add_path(&path, None, true);
                        }
                    } else {
                        guard.add_path(&path, None, true);
                    }
                }
                DebouncedEvent::Remove(path) => {
                    println!("File removed: {:?}", path);
                    _ = db.delete_by_entry(path.file_name().unwrap().to_str().unwrap()); // 数据库删除
                    let mut guard = CACHER.lock().unwrap();
                    guard.remove_path(&path); // 缓存删除
                }
                DebouncedEvent::Rename(old_path, new_path) => {
                    println!("File renamed from {:?} to {:?}", old_path, new_path);
                }
                DebouncedEvent::NoticeWrite(path) => {
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
