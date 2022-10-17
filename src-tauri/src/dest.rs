use std::sync::Mutex;
use crate::redis_manager::Manager;
pub struct AppState {
  redis_client: Mutex<Manager>,
}

pub struct Payload {
    message: String,
}
