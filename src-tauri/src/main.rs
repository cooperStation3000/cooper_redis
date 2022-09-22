#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use redis::Commands;

mod redis_manager;

extern crate redis;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str)-> &str{
    let mut con = redis_manager::get_redis_connection("redis://127.0.0.1:6379/0");
    let _: () = con.set("test", name).expect("set error");
    name
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
