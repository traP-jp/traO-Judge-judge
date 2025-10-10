use axum::async_trait;
use domain::external::mail::MailClient;
use lettre::Address;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SentMail {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// Mock implementation of MailClient that stores sent emails in memory
/// instead of actually sending them via SMTP.
#[derive(Clone)]
pub struct MailClientMock {
    sent_emails: Arc<Mutex<Vec<SentMail>>>,
}

impl MailClientMock {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get all emails that have been "sent" through this mock client
    pub async fn get_sent_emails(&self) -> Vec<SentMail> {
        self.sent_emails.lock().await.clone()
    }

    /// Clear all stored emails
    pub async fn clear(&self) {
        self.sent_emails.lock().await.clear();
    }
}

impl Default for MailClientMock {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MailClient for MailClientMock {
    async fn send_mail(&self, send_to: Address, subject: &str, body: &str) -> anyhow::Result<()> {
        let mail = SentMail {
            to: send_to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        };
        
        tracing::debug!("Mock email sent to {}: {}", mail.to, mail.subject);
        
        self.sent_emails.lock().await.push(mail);
        Ok(())
    }
}
