/// Example demonstrating basic usage of backend_infra_mock
/// Run with: cargo run --package backend_infra_mock --example basic_usage
use backend_infra_mock::auth::AuthRepositoryMock;
use backend_infra_mock::mail::MailClientMock;
use domain::external::mail::MailClient;
use domain::model::user::UserId;
use domain::repository::auth::AuthRepository;
use lettre::Address;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Backend Infrastructure Mocks Example ===\n");

    // === Mail Mock Example ===
    println!("--- Mail Mock Example ---");
    let mail_client = MailClientMock::new();

    // Send some test emails
    for i in 1..=3 {
        let addr = format!("user{}@example.com", i).parse::<Address>().unwrap();
        mail_client
            .send_mail(
                addr,
                &format!("Test Subject {}", i),
                &format!("Test Body {}", i),
            )
            .await?;
        println!("✓ Email {} sent", i);
    }

    // Inspect sent emails
    let sent_emails = mail_client.get_sent_emails().await;
    println!("\nTotal emails sent: {}", sent_emails.len());
    for (idx, email) in sent_emails.iter().enumerate() {
        println!(
            "  Email {}: To={}, Subject={}",
            idx + 1,
            email.to,
            email.subject
        );
    }

    // === Auth Repository Mock Example ===
    println!("\n--- Auth Repository Mock Example ---");
    let auth_repo = AuthRepositoryMock::new();
    let user_id = UserId(Uuid::new_v4());
    println!("Created test user with ID: {}", user_id.0);

    // Password authentication
    println!("\n1. Password Authentication:");
    auth_repo
        .save_user_password(user_id, "secure_password")
        .await?;
    println!("✓ Password saved");

    let is_valid = auth_repo
        .verify_user_password(user_id, "secure_password")
        .await?;
    println!("✓ Password verification: {}", is_valid);

    let is_invalid = auth_repo
        .verify_user_password(user_id, "wrong_password")
        .await?;
    println!("✓ Wrong password verification: {}", is_invalid);

    // Google OAuth
    println!("\n2. Google OAuth:");
    let google_url = auth_repo.get_google_oauth2_url("login").await?;
    println!("✓ OAuth URL: {}", google_url);

    let google_oauth_id = auth_repo
        .get_google_oauth_by_authorize_code("auth_code_123", "login")
        .await?;
    println!("✓ OAuth ID from code: {}", google_oauth_id);

    auth_repo
        .save_user_google_oauth(user_id, &google_oauth_id)
        .await?;
    println!("✓ Google OAuth saved for user");

    let verified = auth_repo.verify_user_google_oauth(user_id).await?;
    println!("✓ Google OAuth verified: {}", verified);

    // GitHub OAuth
    println!("\n3. GitHub OAuth:");
    let github_oauth_id = auth_repo
        .get_github_oauth_by_authorize_code("github_code", "signup")
        .await?;
    auth_repo
        .save_user_github_oauth(user_id, &github_oauth_id)
        .await?;
    println!("✓ GitHub OAuth saved");

    // traQ OAuth
    println!("\n4. traQ OAuth:");
    auth_repo
        .save_user_traq_oauth(user_id, "traq_oauth_xyz")
        .await?;
    println!("✓ traQ OAuth saved");

    // Count authentication methods
    let count = auth_repo.count_authentication_methods(user_id).await?;
    println!("\n✓ Total authentication methods for user: {}", count);
    println!("  (password, Google OAuth, GitHub OAuth, traQ OAuth)");

    // Lookup user by OAuth
    println!("\n5. User Lookup by OAuth:");
    let found_user = auth_repo
        .get_user_id_by_google_oauth(&google_oauth_id)
        .await?;
    match found_user {
        Some(id) => println!("✓ Found user by Google OAuth: {}", id.0),
        None => println!("✗ User not found"),
    }

    println!("\n=== Example completed successfully ===");

    Ok(())
}
