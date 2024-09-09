use std::{sync::mpsc, time::Duration};

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::db::Database;
use crate::db::SqliteDatabase;

pub fn file_checker(db: &SqliteDatabase) {
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
                DebouncedEvent::Create(path) => {
                    println!("File created: {:?}", path);
                    _ = db.insert_event(&path.display().to_string());
                }
                DebouncedEvent::Write(path) => {
                    println!("File modified: {:?}", path);
                    _ = db.insert_event(&path.display().to_string());
                }
                DebouncedEvent::Remove(path) => {
                    println!("File removed: {:?}", path);
                    _ = db.insert_event(&path.display().to_string());
                }
                DebouncedEvent::Rename(old_path, new_path) => {
                    println!("File renamed from {:?} to {:?}", old_path, new_path);
                    _ = db.insert_event(&new_path.display().to_string());
                }
                DebouncedEvent::NoticeWrite(path) => {
                    println!("File notice write: {:?}", path);
                    _ = db.insert_event(&path.display().to_string());
                }
                DebouncedEvent::NoticeRemove(path) => {
                    println!("File notice remove: {:?}", path);
                    _ = db.insert_event(&path.display().to_string());
                }
                DebouncedEvent::Chmod(path) => {
                    println!("File permissions changed: {:?}", path);
                    _ = db.insert_event(&path.display().to_string());
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
