use axum::async_trait;

#[async_trait]
pub trait ObjectStrageClient {
    async fn upload(&self, file_name: &str, data: &str) -> anyhow::Result<()>;
    async fn download(&self, file_name: &str) -> anyhow::Result<String>;
    async fn delete(&self, file_name: &str) -> anyhow::Result<()>;
}
