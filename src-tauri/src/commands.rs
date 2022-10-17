use crate::dest::AppState;

#[tauri::command]
pub fn get_redis_info(state: State<AppState>) -> String {
    println!("get_redis_info");
    let cli = state.redis_client.lock().unwrap();
    let info = cli.client.get_connection_info();
    let mut conm = cli.client.get_connection().unwrap();
    conm.req_command(&redis::Cmd::set("12312", "12312"))
    .expect("setError");
    info.addr.to_string()
}




