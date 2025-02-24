use axum::async_trait;
use lettre::Address;

#[async_trait]
pub trait MailClient {
    async fn send_mail(&self, send_to: Address, subject: &str, body: &str) -> anyhow::Result<()>;
}
