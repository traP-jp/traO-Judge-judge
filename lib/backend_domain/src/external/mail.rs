use axum::async_trait;
use lettre::Address;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait MailClient {
    async fn send_mail(&self, send_to: Address, subject: &str, body: &str) -> anyhow::Result<()>;
}
