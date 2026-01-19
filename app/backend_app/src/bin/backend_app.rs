use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false) // neoshowcase で export logs するときに崩れなくするため
        .init();

    backend_app::run().await?;

    Ok(())
}
