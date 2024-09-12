use std::{
    path::PathBuf,
    sync::{mpsc::Receiver, Arc, Mutex},
};

use crate::db::{Database, EntryMeta};

#[allow(unused)]
pub enum DbAction {
    CREATE,
    FIND,
    DELETE,
    UPDATE,
}

pub struct DbMsg {
    pub action: DbAction,
    pub entry: Option<String>,
    pub path: Option<PathBuf>,
    pub meta: Option<EntryMeta>,
}

pub fn db_writer(db: Arc<Mutex<dyn Database>>, db_receiver: Receiver<DbMsg>) {
    loop {
        match db_receiver.recv() {
            Ok(msg) => match msg.action {
                DbAction::CREATE => {
                    println!("create: {:?}", msg.path);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.insert_rec(&msg.path.unwrap(), &msg.meta.unwrap());
                }
                DbAction::FIND => {
                    println!("find: nothing to do")
                }
                DbAction::DELETE => {
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.delete_by_entry(&msg.entry.unwrap());
                }
                DbAction::UPDATE => {
                    println!("update: {:?}", msg.entry);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.update_meta(&msg.path.unwrap(), &msg.meta.unwrap());
                }
            },
            Err(e) => {
                println!("error: {:?}", e);
            }
        }
    }
}
