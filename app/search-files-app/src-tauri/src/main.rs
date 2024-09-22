// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;

use app::server::launch_file_elf;
use app::shortcut::register_shorcut;
use app::tray;

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
            // 启动后台缓存服务
            launch_file_elf(app);

            // 注册托盘
            #[cfg(desktop)]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }

            // 注册快捷键
            register_shorcut(app)
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![open_file, open_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
