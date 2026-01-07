use axum::async_trait;
use lettre::{
    Address, Message, SmtpTransport, Transport,
    message::{Mailbox, SinglePart, header},
    transport::smtp::authentication::Credentials,
};

use domain::external::mail::MailClient;

#[derive(Clone)]
pub struct MailClientImpl {
    mailer: SmtpTransport,
}

impl MailClientImpl {
    pub fn new() -> anyhow::Result<Self> {
        let app_address = std::env::var("MAIL_ADDRESS").unwrap();
        let app_password = std::env::var("MAIL_PASSWORD").unwrap();
        let smtp = std::env::var("MAIL_SMTP").unwrap();

        let credentials = Credentials::new(app_address.clone(), app_password.clone());

        let mailer = SmtpTransport::starttls_relay(&smtp)?
            .credentials(credentials)
            .build();

        Ok(Self { mailer })
    }
}

#[async_trait]
impl MailClient for MailClientImpl {
    async fn send_mail(&self, send_to: Address, subject: &str, body: &str) -> anyhow::Result<()> {
        let email = Message::builder()
            .from(Mailbox::new(
                Some("traOJudge".to_string()),
                std::env::var("MAIL_ADDRESS")
                    .unwrap()
                    .parse::<Address>()
                    .unwrap(),
            ))
            .to(Mailbox::new(None, send_to))
            .subject(subject)
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(body.to_string()),
            )?;

        let mailer = self.mailer.clone();
        tokio::task::spawn_blocking(move || {
            mailer.send(&email)
        });

        Ok(())
    }
}
