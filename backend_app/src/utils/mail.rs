use lettre::{
    message::{header, Mailbox, SinglePart},
    transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};

pub async fn send_email(send_to: Address, subject: &str, message: &str) -> anyhow::Result<()> {
    let app_address = std::env::var("MAIL_ADDRESS").unwrap();
    let app_password = std::env::var("MAIL_PASSWORD").unwrap();
    let smtp = "smtp.gmail.com";

    let email = Message::builder()
        .from(Mailbox::new(
            Some("traOJudge".to_string()),
            app_address.parse::<Address>().unwrap(),
        ))
        .to(Mailbox::new(None, send_to))
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(message.to_string()),
        )?;

    let credentials = Credentials::new(app_address, app_password);

    let mailer = SmtpTransport::starttls_relay(smtp)?
        .credentials(credentials)
        .build();

    mailer.send(&email)?;

    Ok(())
}
