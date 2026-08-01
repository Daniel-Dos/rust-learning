use app_rust::rest::router_user;
use app_rust::rest::server_user::server;
use tracing::log::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|e| {
                warn!("Não foi possivel ler a variavel RUST_LOG, ira seguir no padrao: {e}");
                tracing_subscriber::EnvFilter::new(
                    "warn,app_rust=trace,reqwest=trace",
                )
            }),
        ).json()
        .init();

    let app = router_user::router_users().await;
    server(app).await?;

    Ok(())
}