use std::{
    fs,
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use crate::db::{Database, EntryMeta, DB};

#[allow(unused)]
pub enum DbAction {
    CREATE(PathBuf, EntryMeta),
    FIND,
    DELETE(PathBuf),
    UPDATE(PathBuf, EntryMeta),
}

use lazy_static::lazy_static;
use log::{debug, error, info};

// 使用 lazy_static 初始化全局的 Arc<Sender> 和 Receiver
lazy_static! {
    pub static ref SENDER: Sender<DbAction> = {
        let (tx, rx) = mpsc::channel();


    // 启动后台数据库写入线程
    std::thread::spawn(move || {
        db_writer(DB.clone(), rx);
    });

    tx // 返回全局共享的 Sender
    };
}

pub fn db_writer(db: Arc<Mutex<dyn Database>>, db_receiver: Receiver<DbAction>) {
    info!("db_writer: start");
    loop {
        match db_receiver.recv() {
            Ok(msg) => match msg {
                DbAction::CREATE(path, meta) => {
                    debug!("db_writer: create: {:?}", &path);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.insert_rec(&path, &meta);
                }
                DbAction::FIND => {
                    debug!("db_writer: find: nothing to do")
                }
                DbAction::DELETE(path) => {
                    if let Ok(path_type) = fs::metadata(&path) {
                        let db_guard: std::sync::MutexGuard<'_, dyn Database> = db.lock().unwrap();

                        if path_type.is_dir() {
                            debug!("db_writer: delete dir: {:?}", &path);

                            // 删除指定路径文件夹下的所有内容
                            let res = db_guard.delete_by_path_prefix(&path);
                            match res {
                                Ok(_) => {
                                    debug!("db_writer: delete dir success")
                                }
                                Err(e) => {
                                    debug!("db_writer: delete error: {:?}", e);
                                }
                            }
                        } else if path_type.is_file() {
                            debug!("db_writer: delete file: {:?}", &path);

                            // 只删除指定的文件路径
                            let res = db_guard.delete_by_path(&path);
                            match res {
                                Ok(_) => {
                                    debug!("db_writer: delete file success")
                                }
                                Err(e) => {
                                    debug!("db_writer: delete error: {:?}", e);
                                }
                            }
                        } else if path_type.is_symlink() {
                            debug!("Unsupported file type: symlink")
                        }
                    }
                }
                DbAction::UPDATE(path, meta) => {
                    debug!("db_writer: update: {:?}", &path);
                    let db_guard = db.lock().unwrap();
                    _ = db_guard.update_meta(&path, &meta);
                }
            },
            Err(e) => {
                error!("db_writer: db_writer error: {:?}", e);
            }
        }
    }
}
