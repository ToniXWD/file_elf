use std::{fs, path::PathBuf, sync::Arc};

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
use tokio::{
    spawn,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

// 使用 lazy_static 初始化全局的 Arc<Sender> 和 Receiver
lazy_static! {
    pub static ref SENDER: Sender<DbAction> = {
        let (tx,  rx) = mpsc::channel(10);
        spawn(db_writer(DB.clone(), rx));

    tx // 返回全局共享的 Sender
    };
}

pub async fn db_writer(db: Arc<Mutex<dyn Database>>, mut db_receiver: Receiver<DbAction>) {
    info!("db_writer: start");
    loop {
        let mpsc_msg = db_receiver.recv().await;
        match mpsc_msg {
            Some(msg) => match msg {
                DbAction::CREATE(path, meta) => {
                    debug!("db_writer: create: {:?}", &path);
                    let db_guard = db.lock().await;
                    _ = db_guard.insert_rec(&path, &meta);
                }
                DbAction::FIND => {
                    debug!("db_writer: find: nothing to do")
                }
                DbAction::DELETE(path) => {
                    if let Ok(path_type) = fs::metadata(&path) {
                        let db_guard = db.lock().await;

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
                    let db_guard = db.lock().await;
                    _ = db_guard.update_meta(&path, &meta);
                }
            },
            None => {
                error!("db_writer: no action received");
            }
        }
    }
}
