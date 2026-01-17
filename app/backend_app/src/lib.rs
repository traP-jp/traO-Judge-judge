use crate::di::DiContainer;
use axum::http;
use infra::provider::Provider;
use tower_http::{
    LatencyUnit,
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub mod di;
pub mod handler;
pub mod model;

pub async fn run() -> anyhow::Result<()> {
    let provider = Provider::new().await.map_err(|e| {
        tracing::error!("Failed to create provider: {}", e);
        e
    })?;
    let di_container = DiContainer::new(provider).await;

    let origins = [
        "http://localhost:3000"
            .parse()
            .expect("Failed to parse hardcoded localhost URL"),
        std::env::var("FRONTEND_URL")
            .map_err(|e| anyhow::anyhow!("FRONTEND_URL env var missing: {}", e))?
            .parse()
            .map_err(|e| anyhow::anyhow!("FRONTEND_URL is invalid: {}", e))?,
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            http::Method::POST,
            http::Method::GET,
            http::Method::DELETE,
            http::Method::PUT,
        ])
        .allow_headers(tower_http::cors::Any);

    let app = handler::make_router(di_container)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("listening on {}", listener.local_addr()?);

    #[cfg(feature = "prod")]
    traq_log::send_info_message(Some("BACKEND APP START"), "サーバーが起動されました。").await;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

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
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C (SIGINT) received, starting graceful shutdown");
        }
        _ = terminate => {
            tracing::info!("SIGTERM received, starting graceful shutdown");
        }
    }

    #[cfg(feature = "prod")]
    traq_log::send_warning_message(
        Some("BACKEND APP SHUTDOWN"),
        "サーバーがシャットダウンされました。",
    )
    .await;
}
