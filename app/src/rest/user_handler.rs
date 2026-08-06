use crate::models::user::User as user_model;
use crate::rest::state::AppState;
use crate::rest::user_request::{UserRequest, UserUpdate};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use tracing::{error, info};
use uuid::Uuid;

pub async fn create_user(State(state): State<AppState>, mut payload: Json<UserRequest>)
                         -> Result<(StatusCode, Json<UserRequest>), (StatusCode, String)> {
    info!("Criando um novo Usuario");
    let user_id = payload.user_id
        .get_or_insert_with(|| Uuid::new_v4().to_string())
        .clone();
    let user = user_model::new(payload.username.clone(),
                               payload.email.clone(), payload.age.clone(), user_id);

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
            user_id: Some(users_list.user_id().to_string()),
        });
    }
    Ok((StatusCode::OK, Json(user_response)))
}

pub async fn get_user(State(state): State<AppState>, Path(username):Path<String>)
    -> Result<(StatusCode, Json<UserRequest>), (StatusCode, String)> {
    info!("Obtendo os dados do usuario: {}", username);

    let user_find = state.user_service.find_user_by_user_id(&username)
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
        user_id:Some(user_find.user_id().to_string()),
    };

    Ok((StatusCode::OK, Json(user_response)))
}

pub async fn delete_user(State(state): State<AppState>,Path(user_id):Path<String>)
    -> Result<(StatusCode, String), (StatusCode, String)> {
    info!("Deletentando o usuario: {}", user_id);

    state.user_service.delete_user_by_user_id(&user_id)
        .await
        .map_err(|e| {
            error!("Erro ao deletar usuario {}: {}", user_id, e);
            match e {
                crate::service::user_service::UserError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            }
        })?;

    Ok((StatusCode::OK, format!("Usuario: {} deletado com sucesso!", user_id)))
}

pub async fn update_user(State(state): State<AppState>, Path(user_id):Path<String>, payload: Json<UserUpdate>)
    -> Result<(StatusCode, String), (StatusCode, String)> {
    info!("Atualizando o email do usuario: {}", user_id);

    let Some(email) = payload.email.as_deref() else {
        return Err((StatusCode::BAD_REQUEST, "Campo 'email' é obrigatório".to_string()));
    };

    state.user_service.update_user_email_by_user_id(&user_id, &email)
        .await
        .map_err(|e| {
            error!("Erro ao atualizar email do usuario {}: {}", user_id, e);
            match e {
                crate::service::user_service::UserError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            }
        })?;

    Ok((StatusCode::OK, format!("Email do usuario: {} atualizado com sucesso!", user_id)))
}