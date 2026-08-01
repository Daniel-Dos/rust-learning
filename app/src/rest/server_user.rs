use axum::Router;
use tracing::info;

pub async fn server(app: Router) -> Result<(), anyhow::Error> {
    info!("Server starting on http://localhost:8080");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    info!("Desligamento gracioso concluído. Servidor encerrado.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Sinal de parada recebido (Ctrl+C). Drenando requisições..."),
        _ = terminate => info!("Sinal de parada recebido (SIGTERM). Drenando requisições..."),
    }
}
