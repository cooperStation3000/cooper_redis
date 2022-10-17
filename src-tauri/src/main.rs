#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::sync::Mutex;
mod redis_manager;
mod commands;
mod dest;

fn main() {
    let redis_manager = redis_manager::Manager::init();
    tauri::Builder::default()
        // .setup(|app| Ok(()))
        .invoke_handler(tauri::generate_handler![
        commands::get_redis_info
        ])
        .manage(dest::AppState {
            redis_client: Mutex::from(redis_manager),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
