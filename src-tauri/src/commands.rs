use crate::dest::AppState;
use redis::{Cmd, ConnectionLike, RedisResult, Value};
use tauri::command;
use tauri::State;

#[command]
pub fn get_redis_info(state: State<AppState>) -> String {
    println!("get_redis_info");
    let cli = state.redis_client.lock().unwrap();
    let info = cli.client.get_connection_info();
    let mut conm = cli.client.get_connection().unwrap();
    conm.req_command(&Cmd::set("12312", "12312"))
        .expect("setError");
    info.addr.to_string()
}

#[command]
pub fn select_db(state: State<AppState>, db: i8) -> i64 {
    println!("select db");
    let cli = state.redis_client.lock().unwrap();
    let mut con = cli.client.get_connection().unwrap();
    con.req_command(&Cmd::new().arg("select").arg(db))
        .expect("选择错误");
    con.get_db()
}
