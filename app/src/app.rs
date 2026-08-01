use sqlx::sqlite::SqlitePoolOptions;
use app_rust::repository::db_sqlite::UserDBSqlite as user_db_sqlite;
use app_rust::rest::router_user;
use app_rust::rest::server_user::server;
use app_rust::rest::state::AppState;
use app_rust::service::user_service::UserService as user_service;
use tracing::error;
use tracing::warn;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|e| {
                warn!("Não foi possivel ler a variavel RUST_LOG, ira seguir no padrao: {e}");
                tracing_subscriber::EnvFilter::new("warn,app_rust=trace,reqwest=trace")
            }),
        )
        .json()
        .init();

    let user_db = user_db_sqlite::new(
        SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite:user-rust.db")
            .await
            .map_err(|e| {
                error!("Erro ao conectar com o banco de dados: {}", e);
                e
            })?
    );

    sqlx::migrate!("./migrations")
        .run(user_db.get_pool())
        .await
        .map_err(|e| {
            error!("Erro ao rodar migrações: {}", e);
            e
        })?;


    let user_servicer = user_service::new(user_db);

    let app_state = AppState {
        user_service: user_servicer,
    };
    let app = router_user::router_users(app_state);
    server(app).await?;

    Ok(())
}
