use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub username: String,
    pub age: i32
}