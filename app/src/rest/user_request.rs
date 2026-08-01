use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub username: String,
    pub age: i32
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub username: Option<String>,
    pub age: Option<i32>,
}