use di::DiContainer;
use infra::provider::Provider;
use tower_http::trace::TraceLayer;

pub mod handler;
pub mod model;

pub async fn run() -> anyhow::Result<()> {
    let provider = Provider::new().await.unwrap();
    let di_container = DiContainer::new(provider).await;

    let app = handler::make_router(di_container).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::debug!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
