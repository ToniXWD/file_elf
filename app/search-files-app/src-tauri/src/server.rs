use tauri::App;
use tauri_plugin_cli::CliExt;
use tokio::runtime::Runtime;

pub fn launch_file_elf(app: &mut App) {
    // let mut need_launch = true;
    // match app.cli().matches() {
    //     Ok(matches) => {
    //         match matches.args.get("split") {
    //             Some(data) => {
    //                 if data.value.as_bool().unwrap() {
    //                     // file_elf在其他地方启动并启动了webserver
    //                     need_launch = false;
    //                     println!("no need to launch the file_elf")
    //                 }
    //             }
    //             None => {
    //                 println!("no need to launch the file_elf");
    //             }
    //         }
    //     }
    //     Err(_) => {
    //         println!("no need to launch the file_elf");
    //     }
    // }
    // if need_launch {
    let rt = Runtime::new().unwrap();
    // 在后台线程中执行异步任务
    std::thread::spawn(move || {
        rt.block_on(async {
            file_elf::launch_elf().await;
        });
    });
    // }
}
