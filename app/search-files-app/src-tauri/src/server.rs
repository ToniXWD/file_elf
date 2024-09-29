use std::thread::spawn;

use log::info;
use tauri::{async_runtime::block_on, App};
use tauri_plugin_cli::CliExt;

pub fn launch_file_elf(app: &mut App) {
    let mut need_launch = true;
    match app.cli().matches() {
        Ok(matches) => {
            match matches.args.get("split") {
                Some(data) => {
                    if data.value.as_bool().unwrap() {
                        // file_elf在其他地方启动并启动了webserver
                        need_launch = false;
                        info!("no need to launch the file_elf")
                    }
                }
                None => {
                    info!("no need to launch the file_elf");
                }
            }
        }
        Err(_) => {
            info!("no need to launch the file_elf");
        }
    }
    if need_launch {
        // 在后台线程中执行异步任务
        spawn(move || {
            block_on(file_elf::launch_elf(false));
        });
    }
}
