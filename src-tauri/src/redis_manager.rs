extern crate redis;

use redis::Connection;

// struct Manager {}
//
// impl Manager {
//     new(){}
// }

pub fn get_redis_connection(db_url: &str) -> Connection {
    let client = redis::Client::open(db_url).expect("redis 连接失败");
    let con = client.get_connection().expect("redis 获取连接失败");
    con
}
