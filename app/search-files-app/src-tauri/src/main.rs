// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::{env, process::Command};

use app::server::launch_file_elf;
use app::shortcut::register_shorcut;
use app::tray;

use file_elf::server::api;
use log::{error, info, trace, warn};

/// 热点文件搜索
#[tauri::command]
fn hot_search(entry: String, is_fuzzy: bool, is_regex: bool) -> Vec<(String, bool)> {
    let res = api::api_hot_search(entry, is_fuzzy, is_regex);
    res
}

/// 正则表达式搜索
#[tauri::command]
fn regex_search(entry: String) -> Vec<(String, bool)> {
    let res = api::api_regex_search(entry);
    res
}

/// 常规搜索
#[tauri::command]
fn search(entry: String, is_fuzzy: bool) -> Vec<(String, bool)> {
    let res = api::api_search(entry, is_fuzzy);
    res
}

/// star_path
#[tauri::command]
fn star_path(path: String) -> bool {
    let res = api::api_star_path(path);
    res
}

/// unstar_path
#[tauri::command]
fn unstar_path(path: String) -> bool {
    let res = api::api_unstar_path(path);
    res
}

/// 打开文件
#[tauri::command]
fn open_file(name: String) {
    trace!("Opening file: {}", name);
    let path = PathBuf::from(name);

    if path.exists() {
        open_directory(&path);
    } else {
        warn!("The provided path does not exist.");
    }
}

/// 判断name是文件还是文件夹, 如果是文件夹则打开资源管理器, 否则打开文件所在目录的资源管理器
#[tauri::command]
fn open_dir(name: String) {
    info!("Opening file or directory: {}", name);
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
        error!("The provided path does not exist.");
    }
}

/// 打开指定的目录
fn open_directory(path: &PathBuf) {
    if let Err(err) = open::that(path) {
        error!("Failed to open directory: {}", err);
    } else {
        info!("Directory opened successfully.");
    }
}

#[tauri::command]
fn open_vscode(path: String) {
    let command = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "code"
    };

    let args: Vec<&str> = if cfg!(target_os = "windows") {
        vec!["/C", "code", &path]
    } else {
        vec![&path]
    };

    let output = Command::new(command).args(&args).output();

    match output {
        Ok(output) => {
            if !output.stderr.is_empty() {
                error!(
                    "Failed to open path: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            } else {
                info!("Directory opened successfully.");
            }
        }
        Err(err) => error!("Failed to open path: {}", err),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .max_file_size(50_000 /* bytes */)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        path: PathBuf::from("."),
                        file_name: Some("search_file_app".to_string()),
                    },
                ))
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .level({
                    match file_elf::config::CONF.database.log_level.as_str() {
                        "trace" => log::LevelFilter::Trace,
                        "debug" => log::LevelFilter::Debug,
                        "info" => log::LevelFilter::Info,
                        "warn" => log::LevelFilter::Warn,
                        "error" => log::LevelFilter::Error,
                        _ => log::LevelFilter::Info,
                    }
                })
                .build(),
        )
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
            match register_shorcut(app) {
                Ok(_) => info!("register shortcut success"),
                Err(e) => error!("register shortcut error: {}", e),
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            open_file,
            open_dir,
            open_vscode,
            hot_search,
            regex_search,
            search,
            star_path,
            unstar_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
