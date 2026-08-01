use crate::models::user::User as user_model;
use crate::repository::db_sqlite::UserDBSqlite as user_db_sqlite;
use crate::rest::state::AppState;
use crate::rest::user_request::UserRequest;
use crate::service::user_service::UserService as user_service;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::{error, info};

pub async fn create_user(State(state): State<AppState>,payload: Json<UserRequest>) -> Result<(StatusCode, Json<UserRequest>), (StatusCode, String)> {
    info!("Criando um novo Usuario");

    info!("Salvando o usuario");
    let user = user_model::new(payload.username.clone(), payload.email.clone(), payload.age.clone());

    state.user_service.create_user(&user).await
        .map(|_| info!("Usuario: {} salvo com sucesso!", payload.username))
        .map_err(|e| {
            error!("Erro ao salvar o usuario: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::CREATED, payload))
}

pub async fn get_users(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<UserRequest>>), (StatusCode, String)> {
    info!("Obtendo todos os usuarios.");

    let users = state.user_service.get_all_users().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut user_response:Vec<UserRequest>  = vec![];

    for users_list in users  {
        user_response.push(UserRequest {
            username: users_list.username().to_string(),
            email: users_list.email().to_string(),
            age: users_list.age().clone(),
        });
    }
    Ok((StatusCode::OK, Json(user_response)))
}