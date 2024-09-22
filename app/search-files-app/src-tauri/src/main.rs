// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use tauri_plugin_cli::CliExt;
use tokio::runtime::Runtime;

/// 打开文件
#[tauri::command]
fn open_file(name: String) {
    println!("Opening file: {}", name);
    let path = PathBuf::from(name);

    if path.exists() {
        open_directory(&path);
    } else {
        println!("The provided path does not exist.");
    }
}

/// 判断name是文件还是文件夹, 如果是文件夹则打开资源管理器, 否则打开文件所在目录的资源管理器
#[tauri::command]
fn open_dir(name: String) {
    println!("Opening file or directory: {}", name);
    let path = PathBuf::from(name);

    if path.exists() {
        if path.is_dir() {
            // 如果是目录，则直接打开该目录
            open_directory(&path);
        } else if path.is_file() {
            // 如果是文件，则尝试打开文件所在的目录
            let parent_dir = path
                .parent()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            open_directory(&parent_dir);
        }
    } else {
        println!("The provided path does not exist.");
    }
}

/// 打开指定的目录
fn open_directory(path: &PathBuf) {
    if let Err(err) = open::that(path) {
        println!("Failed to open directory: {}", err);
    } else {
        println!("Directory opened successfully.");
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            let mut need_launch = true;
            match app.cli().matches() {
                Ok(matches) => {
                    match matches.args.get("split") {
                        Some(data) => {
                            if data.value.as_bool().unwrap() {
                                // file_elf在其他地方启动并启动了webserver
                                need_launch = false;
                                println!("no need to launch the file_elf")
                            }
                        }
                        None => {
                            println!("no need to launch the file_elf");
                        }
                    }
                }
                Err(_) => {
                    println!("no need to launch the file_elf");
                }
            }
            if need_launch {
                let rt = Runtime::new().unwrap();
                // 在后台线程中执行异步任务
                std::thread::spawn(move || {
                    rt.block_on(async {
                        file_elf::launch_elf().await;
                    });
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_file, open_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
