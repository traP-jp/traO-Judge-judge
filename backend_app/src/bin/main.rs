use anyhow::Ok;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use trao_judge_backend as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .init();

    let app_state = lib::Repository::connect().await?;
    app_state.migrate().await?;

    let app = lib::make_router(app_state).layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::debug!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
