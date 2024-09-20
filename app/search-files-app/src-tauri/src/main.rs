// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

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

fn launch_file_elf(has_window: bool) {
    // 获取当前目录
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // 判断操作系统，选择正确的文件名
    #[cfg(target_os = "windows")]
    let elf_file = "file_elf.exe";

    #[cfg(not(target_os = "windows"))]
    let elf_file = "file_elf";

    // 生成文件路径
    let elf_path = current_dir.join(elf_file);

    // 如果文件存在，则启动子进程
    if elf_path.exists() {
        println!("Found backend executable: {}", elf_file);

        #[cfg(target_os = "windows")]
        {
            let result;

            if has_window {
                result = Command::new(elf_path).spawn(); // 启动子进程
            } else {
                result = Command::new(elf_path)
                    .creation_flags(0x08000000) // CREATE_NO_WINDOW flag for Windows
                    .spawn(); // 启动子进程
            }
            match result {
                Ok(_) => println!("Backend process started successfully."),
                Err(e) => eprintln!("Failed to start backend process: {}", e),
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let result = Command::new(elf_path).spawn(); // 启动子进程 (非Windows平台)
            match result {
                Ok(_) => println!("Backend process started successfully."),
                Err(e) => eprintln!("Failed to start backend process: {}", e),
            }
        }
    } else {
        eprintln!("Backend executable not found: {}", elf_file);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_file, open_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
