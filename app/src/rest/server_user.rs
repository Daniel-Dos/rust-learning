use axum::Router;
use tracing::info;

pub async fn server(app:Router) -> Result<(), anyhow::Error> {
    info!("Server starting on http://localhost:8080");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}