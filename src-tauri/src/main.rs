#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use redis::ConnectionLike;
use redis_manager::Manager;
use std::sync::Mutex;
use tauri::State;
mod redis_manager;
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

struct AppState {
    redis_client: Mutex<Manager>,
}

fn main() {
    let redis_manager = redis_manager::Manager::init();
    tauri::Builder::default()
        // .setup(|app| Ok(()))
        .invoke_handler(tauri::generate_handler![get_redis_info,])
        .manage(AppState {
            redis_client: Mutex::from(redis_manager),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_redis_info(state: State<AppState>) -> String {
    println!("get_redis_info");
    let cli = state.redis_client.lock().unwrap();
    let info = cli.client.get_connection_info();
    let mut conm = cli.client.get_connection().unwrap();
    conm.req_command(&redis::Cmd::set("12312", "12312"))
        .expect("setError");
    info.addr.to_string()
}
