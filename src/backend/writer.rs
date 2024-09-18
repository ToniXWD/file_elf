use std::{
    fs,
    path::PathBuf,
    sync::{mpsc::Receiver, Arc, Mutex},
};

use crate::db::{Database, EntryMeta};

#[allow(unused)]
pub enum DbAction {
    CREATE(PathBuf, EntryMeta),
    FIND,
    DELETE(PathBuf),
    UPDATE(PathBuf, EntryMeta),
}

pub fn db_writer(db: Arc<Mutex<dyn Database>>, db_receiver: Receiver<DbAction>) {
    println!("db_writer: start");
    loop {
        match db_receiver.recv() {
            Ok(msg) => match msg {
                DbAction::CREATE(path, meta) => {
                    println!("db_writer: create: {:?}", &path);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.insert_rec(&path, &meta);
                }
                DbAction::FIND => {
                    println!("db_writer: find: nothing to do")
                }
                DbAction::DELETE(path) => {
                    if let Ok(path_type) = fs::metadata(&path) {
                        let db_guard: std::sync::MutexGuard<'_, dyn Database> = db.lock().unwrap();

                        if path_type.is_dir() {
                            println!("db_writer: delete dir: {:?}", &path);

                            // 删除指定路径文件夹下的所有内容
                            let res = db_guard.delete_by_path_prefix(&path);
                            match res {
                                Ok(_) => {
                                    println!("db_writer: delete dir success")
                                }
                                Err(e) => {
                                    println!("db_writer: delete error: {:?}", e);
                                }
                            }
                        } else if path_type.is_file() {
                            println!("db_writer: delete file: {:?}", &path);

                            // 只删除指定的文件路径
                            let res = db_guard.delete_by_path(&path);
                            match res {
                                Ok(_) => {
                                    println!("db_writer: delete file success")
                                }
                                Err(e) => {
                                    println!("db_writer: delete error: {:?}", e);
                                }
                            }
                        } else if path_type.is_symlink() {
                            println!("Unsupported file type: symlink")
                        }
                    }
                }
                DbAction::UPDATE(path, meta) => {
                    println!("db_writer: update: {:?}", &path);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.update_meta(&path, &meta);
                }
            },
            Err(e) => {
                println!("db_writer: db_writer error: {:?}", e);
            }
        }
    }
}
