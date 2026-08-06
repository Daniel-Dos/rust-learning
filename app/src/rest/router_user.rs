use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post};
use crate::rest::state::AppState;
use crate::rest::user_handler::{create_user, delete_user, get_user, get_users, update_user};

pub fn router_users(app: AppState) -> Router {
    Router::new()
        .route("/api/users", post(create_user).get(get_users))
        .route("/api/users/{user_id}", get(get_user).delete(delete_user).put(update_user))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(app)
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn ready() -> StatusCode {
    StatusCode::OK
}
