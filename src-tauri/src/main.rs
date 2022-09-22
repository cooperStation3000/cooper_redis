#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod redis_manager;

extern crate redis;

use redis::Commands;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str)  {
    let mut con = redis_manager::RedisManager::get_redis_connection("redis://127.0.0.1:6379/0").expect("error");
    let _: () = con.set("test", name).expect("set error");
}

fn main() {
    let _: () = con.set("123", "123").expect("1231");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
