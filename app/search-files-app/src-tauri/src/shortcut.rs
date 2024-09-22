use tauri::{App, Manager};

pub fn register_shorcut(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    #[cfg(desktop)]
    {
        use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["ctrl+f1"])?
                .with_handler(|_app, shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        if shortcut.matches(Modifiers::CONTROL, Code::F1) {
                            println!("ctrl+f1 was pressed");
                            if let Some(window) = _app.get_webview_window("main") {
                                if window.is_visible().is_ok() && !window.is_visible().unwrap() {
                                    println!("try to show the window");
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                } else {
                                    println!("try to hide the window");
                                    let _ = window.hide();
                                }
                            } else {
                                println!("window not found");
                            }
                        }
                    }
                })
                .build(),
        )?;
    }
    Ok(())
}
