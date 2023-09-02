#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dispatchers;
mod export;
mod init;
mod js_api;
mod tauri_bridge;
mod tauri_subscription;
mod tauri_webview;

use crate::export::prelude::*;
use tauri::Manager;

fn main() {
    let should_open_dev_tools: bool = std::env::var("TAURI_DEV_TOOLS").is_ok();

    let (signal, signal_receiver) = TauriBridge::new();

    tauri::Builder::default()
        .manage(signal)
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();

            if should_open_dev_tools {
                main_window.open_devtools();
            }

            init::run(main_window, signal_receiver);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            webview_did_mount,
            webview_did_subscribe,
            webview_ecs_event,
            webview_input_event,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
