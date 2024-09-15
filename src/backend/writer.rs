use std::{
    path::PathBuf,
    sync::{mpsc::Receiver, Arc, Mutex},
};

use crate::db::{Database, EntryMeta};

#[allow(unused)]
pub enum DbAction {
    CREATE(PathBuf, EntryMeta),
    FIND,
    DELETE(String),
    UPDATE(PathBuf, EntryMeta),
}

pub fn db_writer(db: Arc<Mutex<dyn Database>>, db_receiver: Receiver<DbAction>) {
    loop {
        match db_receiver.recv() {
            Ok(msg) => match msg {
                DbAction::CREATE(path, meta) => {
                    println!("create: {:?}", &path);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.insert_rec(&path, &meta);
                }
                DbAction::FIND => {
                    println!("find: nothing to do")
                }
                DbAction::DELETE(entry_str) => {
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.delete_by_entry(&entry_str);
                }
                DbAction::UPDATE(path, meta) => {
                    println!("update: {:?}", &path);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.update_meta(&path, &meta);
                }
            },
            Err(e) => {
                println!("db_writer error: {:?}", e);
            }
        }
    }
}
