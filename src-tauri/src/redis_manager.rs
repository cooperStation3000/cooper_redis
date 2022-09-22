extern crate redis;

use redis::Connection;

pub struct RedisManager {}

impl RedisManager {
    pub fn get_redis_connection(db_url: &str) -> Result<Connection, E> {
        let mut  client = redis::Client::open(db_url).expect("redis 连接失败");
        let mut con = client.get_connection().expect("redis 获取连接失败");
        Ok(con)
    }
}
