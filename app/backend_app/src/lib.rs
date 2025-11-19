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

    tracing::debug!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
