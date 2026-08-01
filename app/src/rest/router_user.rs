use axum::Router;
use axum::routing::{get, post};
use crate::rest::state::AppState;
use crate::rest::user_handler::{create_user, get_users};

pub async fn router_users(app:AppState) -> Router {
    Router::new().route("/api/users", post(create_user).get(get_users)).with_state(app)
}