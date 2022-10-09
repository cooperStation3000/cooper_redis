use redis::Client;

pub struct Manager {
    pub client: Client,
}

static REDIS_URL: &str = "redis://127.0.0.1:6379";

impl Manager {
    pub fn init() -> Self {
        let client = redis::Client::open(REDIS_URL).expect("redis 连接失败");
        Manager { client }
    }
}
