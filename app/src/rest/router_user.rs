use axum::Router;
use axum::routing::{get, post};
use crate::rest::user_handler::{create_user, get_users};

pub async fn router_users() -> Router {
    Router::new().route("/api/users", post(create_user).get(get_users))
}