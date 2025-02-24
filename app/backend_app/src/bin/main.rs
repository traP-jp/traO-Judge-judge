use tracing_subscriber::EnvFilter;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    backend_app::run().await?;

    Ok(())
}