mod actions;
mod p2p;

#[derive(serde::Serialize, Clone)]
pub struct DropPayload {
    pub id: String,
    pub item_type: String,
    pub content: String,
    pub preview_path: Option<String>,
}

#[cfg(target_os = "windows")]
mod drop_target;

#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
#[cfg(target_os = "windows")]
use window_vibrancy::{apply_mica, apply_acrylic, apply_blur};

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            p2p::init_p2p(app_handle);
            
            // Apply vibrancy and Drag-IN
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "macos")]
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

                #[cfg(target_os = "windows")]
                {
                    apply_blur(&window, Some((18, 18, 18, 125)))
                        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
                    
                    if let Ok(hwnd) = window.hwnd() {
                        let app_handle = app.handle().clone();
                        let target = drop_target::DropTarget::new(windows::Win32::Foundation::HWND(hwnd.0 as _), app_handle);
                        if let Err(e) = target.register() {
                            println!("Failed to register OLE drag drop: {:?}", e);
                        }
                    }
                }
            }

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_drag::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            actions::compress_image,
            actions::create_zip,
            actions::clean_url,
            actions::generate_qr,
            actions::cleanup_temp_file,
            p2p::send_to_peer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
