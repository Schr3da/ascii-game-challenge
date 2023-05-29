#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod export;
mod init;
mod js_api;
mod signal;
mod subscription;

use crate::export::prelude::*;
use tauri::Manager;

fn main() {
    let (signal, signal_receiver) = JsSignal::new();

    tauri::Builder::default()
        .manage(signal)
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            init::run(main_window, signal_receiver);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![did_webview_mount])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
