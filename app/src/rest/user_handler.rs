use crate::models::user::User as user_model;
use crate::rest::state::AppState;
use crate::rest::user_request::UserRequest;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use tracing::{error, info};

pub async fn create_user(State(state): State<AppState>,payload: Json<UserRequest>)
    -> Result<(StatusCode, Json<UserRequest>), (StatusCode, String)> {
    info!("Criando um novo Usuario");

    info!("Salvando o usuario");
    let user = user_model::new(payload.username.clone(),
                                      payload.email.clone(), payload.age.clone());

    state.user_service.create_user(&user).await
        .map(|_| info!("Usuario: {} salvo com sucesso!", payload.username))
        .map_err(|e| {
            error!("Erro ao salvar o usuario: {}", e);
            match e {
                crate::service::user_service::UserError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            }
        })?;

    Ok((StatusCode::CREATED, payload))
}

pub async fn get_users(State(state): State<AppState>)
    -> Result<(StatusCode, Json<Vec<UserRequest>>), (StatusCode, String)> {
    info!("Obtendo todos os usuarios.");

    let users = state.user_service.get_all_users()
        .await
        .map_err(|e| {
            error!("Erro ao buscar usuarios: {}", e);
            match e {
                crate::service::user_service::UserError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            }
        })?;

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

pub async fn get_user(State(state): State<AppState>, Path(username):Path<String>)
    -> Result<(StatusCode, Json<UserRequest>), (StatusCode, String)> {
    info!("Obtendo os dados do usuario: {}", username);

    let user_find = state.user_service.find_user_by_username(&username)
        .await
        .map_err(|e| {
            error!("Erro ao buscar usuario {}: {}", username, e);
            match e {
                crate::service::user_service::UserError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            }
        })?;

    let user_response = UserRequest {
        username: user_find.username().to_string(),
        email: user_find.email().to_string(),
        age: user_find.age().clone(),
    };

    Ok((StatusCode::OK, Json(user_response)))
}

pub async fn delete_user(State(state): State<AppState>,Path(username):Path<String>)
    -> Result<(StatusCode, String), (StatusCode, String)> {
    info!("Deletentando o usuario: {}", username);

    state.user_service.delete_user_by_username(&username)
        .await
        .map_err(|e| {
            error!("Erro ao deletar usuario {}: {}", username, e);
            match e {
                crate::service::user_service::UserError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            }
        })?;

    Ok((StatusCode::OK, format!("Usuario: {} deletado com sucesso!", username)))
}