use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub user_id: Option<String>,
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