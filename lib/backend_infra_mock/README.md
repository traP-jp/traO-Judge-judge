# Backend Infrastructure Mocks

This library provides mock implementations of backend infrastructure components for testing and development purposes.

## Components

### MailClientMock

A mock implementation of `MailClient` that stores sent emails in memory instead of actually sending them via SMTP.

**Features:**
- Stores all sent emails in memory for inspection
- Useful for testing email functionality without requiring SMTP configuration
- Can retrieve sent emails for verification
- Can clear stored emails

**Example usage:**

```rust
use backend_infra_mock::mail::MailClientMock;
use domain::external::mail::MailClient;
use lettre::Address;

#[tokio::main]
async fn main() {
    let mail_client = MailClientMock::new();
    
    // Send an email
    let addr = "user@example.com".parse::<Address>().unwrap();
    mail_client.send_mail(addr, "Subject", "Body").await.unwrap();
    
    // Check sent emails
    let sent = mail_client.get_sent_emails().await;
    println!("Sent {} emails", sent.len());
    
    // Clear stored emails
    mail_client.clear().await;
}
```

### AuthRepositoryMock

A mock implementation of `AuthRepository` that stores authentication data in memory instead of a database.

**Features:**
- Supports password authentication
- Supports OAuth authentication (Google, GitHub, traQ)
- Stores all data in memory using HashMap
- Returns mock OAuth URLs for testing
- Useful for testing authentication flows without external OAuth providers

**Example usage:**

```rust
use backend_infra_mock::auth::AuthRepositoryMock;
use domain::repository::auth::AuthRepository;
use domain::model::user::UserId;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let auth_repo = AuthRepositoryMock::new();
    let user_id = UserId(Uuid::new_v4());
    
    // Password operations
    auth_repo.save_user_password(user_id, "password123").await.unwrap();
    let valid = auth_repo.verify_user_password(user_id, "password123").await.unwrap();
    println!("Password valid: {}", valid);
    
    // OAuth operations
    let oauth_url = auth_repo.get_google_oauth2_url("login").await.unwrap();
    println!("OAuth URL: {}", oauth_url);
    
    let oauth_id = auth_repo.get_google_oauth_by_authorize_code("code123", "login").await.unwrap();
    auth_repo.save_user_google_oauth(user_id, &oauth_id).await.unwrap();
    
    // Count authentication methods
    let count = auth_repo.count_authentication_methods(user_id).await.unwrap();
    println!("Authentication methods: {}", count);
}
```

## Use Cases

1. **Unit Testing**: Use these mocks in unit tests to avoid external dependencies
2. **Integration Testing**: Test authentication flows without real OAuth providers
3. **Development**: Run the application without SMTP or OAuth configuration
4. **CI/CD**: Run tests in CI/CD pipelines without external service dependencies

## Running Tests

```bash
cargo test --package backend_infra_mock
```

## Implementation Notes

- All data is stored in memory and will be lost when the program terminates
- OAuth URLs returned are mock URLs for testing purposes only
- Password verification in the mock does simple string comparison (no bcrypt)
- Thread-safe using `Arc<Mutex<>>` for concurrent access
