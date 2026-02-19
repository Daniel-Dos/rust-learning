mod repository;
mod models;
mod utils;
mod service;

use crate::utils::utils::{generate_random_email, generate_random_username, generate_random_number};
use anyhow::{anyhow, Result};
use crate::repository::db_sqlite::UserDBSqlite as user_db_sqlite;
use crate::service::user_service::UserService as user_service;
use tracing::{error, info};
use crate::models::user::User as user_model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Iniciando o projeto");

    if let Err(e) = run().await {
        eprintln!("Fatal error: {:#}", e);
        error!("Fatal error: {:#}", e);
    }

    info!("Fim do projeto");
}

async fn run() -> Result<()> {
    let user_db = user_db_sqlite::init_db().await?;
    let user_service = user_service::new(user_db);

    info!("criado o usuario.");
    let user1 = user_model::new(String::from(generate_random_username()),
                                     String::from(generate_random_email()),
                                     generate_random_number());
    info!("Salvando o usuario");
    user_service.create_user(&user1).await
        .map(|_| info!("Usuario: {} salvo com sucesso!", user1.username()))
        .map_err(|e| {
            error!("Erro ao salvar o usuario: {}", e);
            e
        })?;

    info!("Buscando todos os usuarios.");
    let users: Vec<user_model> = user_service.get_all_users().await
        .map(|users|{ info!("Usuarios: {:#?} recuperados com sucesso!", &users);
            users
        })
        .map_err(|e| {
            error!("Erro ao buscar os usuarios {}", e);
            e
        })?;

    info!("Update usuario.");
    let user_id = users.first().ok_or_else(|| anyhow!("Nenhum usuario encontrado!"))?.id();
    let novo_email = generate_random_email();

    user_service.update_user_email(user_id, &novo_email).await
        .map(|_| info!("Usuario: {} atualizado com sucesso!", user_id))
        .map_err(|e| {
            error!("Erro ao atualizar o usuario: {}", e);
            e
        })?;

    info!("Deletando usuario.");
    user_service.delete_user(user_id).await
        .map(|_| info!("Usuario: {} deletado com sucesso!", user_id))
        .map_err(|e| {
            error!("Erro ao deletar o usuario: {}", e);
            e
        })?;

    Ok(())
}