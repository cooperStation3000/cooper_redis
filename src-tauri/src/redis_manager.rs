use redis::Client;

pub struct Manager {
    pub client: Client,
}

impl Manager {
    pub fn init() -> Self {
        let client = redis::Client::open("redis://127.0.0.1:6379").expect("redis 连接失败");
        Manager { client }
    }
}
