use axum::async_trait;
use domain::model::auth::UserAuthentication;
use domain::{model::user::UserId, repository::auth::AuthRepository};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
struct UserAuth {
    password: Option<String>,
    email: Option<String>,
    google_oauth: Option<String>,
    github_oauth: Option<String>,
    traq_oauth: Option<String>,
}

/// Mock implementation of AuthRepository for testing and development
/// Stores authentication data in memory instead of a database.
#[derive(Clone)]
pub struct AuthRepositoryMock {
    users: Arc<Mutex<HashMap<UserId, UserAuth>>>,
    email_to_user: Arc<Mutex<HashMap<String, UserId>>>,
    google_oauth_to_user: Arc<Mutex<HashMap<String, UserId>>>,
    github_oauth_to_user: Arc<Mutex<HashMap<String, UserId>>>,
    traq_oauth_to_user: Arc<Mutex<HashMap<String, UserId>>>,
}

impl AuthRepositoryMock {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            email_to_user: Arc::new(Mutex::new(HashMap::new())),
            google_oauth_to_user: Arc::new(Mutex::new(HashMap::new())),
            github_oauth_to_user: Arc::new(Mutex::new(HashMap::new())),
            traq_oauth_to_user: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn ensure_user_exists(&self, users: &mut HashMap<UserId, UserAuth>, id: UserId) {
        users.entry(id).or_insert_with(|| UserAuth {
            password: None,
            email: None,
            google_oauth: None,
            github_oauth: None,
            traq_oauth: None,
        });
    }
}

impl Default for AuthRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryMock {
    async fn get_authentication_by_user_id(
        &self,
        id: UserId,
    ) -> anyhow::Result<UserAuthentication> {
        let users = self.users.lock().await;
        if let Some(auth) = users.get(&id) {
            Ok(UserAuthentication {
                email: auth.email.clone(),
                google_oauth: auth.google_oauth.clone(),
                github_oauth: auth.github_oauth.clone(),
                traq_oauth: auth.traq_oauth.clone(),
            })
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    async fn count_authentication_methods(&self, id: UserId) -> anyhow::Result<i64> {
        let users = self.users.lock().await;
        if let Some(auth) = users.get(&id) {
            let count = auth.password.is_some() as i64
                + auth.google_oauth.is_some() as i64
                + auth.github_oauth.is_some() as i64
                + auth.traq_oauth.is_some() as i64;
            Ok(count)
        } else {
            Ok(0)
        }
    }

    async fn save_user_email_and_password(
        &self,
        id: UserId,
        email: &str,
        password: &str,
    ) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        self.ensure_user_exists(&mut users, id);
        users.get_mut(&id).unwrap().email = Some(email.to_string());
        users.get_mut(&id).unwrap().password = Some(password.to_string());
        Ok(())
    }

    async fn is_exist_email(&self, email: &str) -> anyhow::Result<bool> {
        let email_to_user = self.email_to_user.lock().await;
        Ok(email_to_user.contains_key(email))
    }

    async fn get_user_id_by_email(&self, email: &str) -> anyhow::Result<Option<UserId>> {
        let email_to_user = self.email_to_user.lock().await;
        Ok(email_to_user.get(email).copied())
    }

    async fn update_user_email(&self, id: UserId, email: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            auth.email = Some(email.to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    async fn update_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            auth.password = Some(password.to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    async fn verify_user_password(&self, id: UserId, password: &str) -> anyhow::Result<bool> {
        let users = self.users.lock().await;
        if let Some(auth) = users.get(&id) {
            if let Some(stored_password) = &auth.password {
                Ok(stored_password == password)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn get_google_oauth2_url(&self, oauth_action: &str) -> anyhow::Result<String> {
        match oauth_action {
            "login" | "signup" | "bind" => Ok(format!(
                "http://mock-google-oauth.example.com/auth?action={}",
                oauth_action
            )),
            _ => Err(anyhow::anyhow!("Invalid OAuth action")),
        }
    }

    async fn get_google_oauth_by_authorize_code(
        &self,
        code: &str,
        oauth_action: &str,
    ) -> anyhow::Result<String> {
        if oauth_action != "login" && oauth_action != "signup" && oauth_action != "bind" {
            return Err(anyhow::anyhow!("Invalid OAuth action"));
        }
        // In mock mode, just return a mock OAuth ID based on the code
        Ok(format!("google_oauth_{}", code))
    }

    async fn save_user_google_oauth(&self, id: UserId, google_oauth: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        self.ensure_user_exists(&mut users, id);
        users.get_mut(&id).unwrap().google_oauth = Some(google_oauth.to_string());

        let mut google_oauth_to_user = self.google_oauth_to_user.lock().await;
        google_oauth_to_user.insert(google_oauth.to_string(), id);

        Ok(())
    }

    async fn update_user_google_oauth(&self, id: UserId, google_oauth: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            // Remove old OAuth mapping if it exists
            if let Some(old_oauth) = &auth.google_oauth {
                let mut google_oauth_to_user = self.google_oauth_to_user.lock().await;
                google_oauth_to_user.remove(old_oauth);
            }

            auth.google_oauth = Some(google_oauth.to_string());

            let mut google_oauth_to_user = self.google_oauth_to_user.lock().await;
            google_oauth_to_user.insert(google_oauth.to_string(), id);

            Ok(())
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    async fn verify_user_google_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let users = self.users.lock().await;
        Ok(users
            .get(&id)
            .and_then(|auth| auth.google_oauth.as_ref())
            .is_some())
    }

    async fn delete_user_google_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            if let Some(oauth) = auth.google_oauth.take() {
                let mut google_oauth_to_user = self.google_oauth_to_user.lock().await;
                google_oauth_to_user.remove(&oauth);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn get_user_id_by_google_oauth(
        &self,
        google_oauth: &str,
    ) -> anyhow::Result<Option<UserId>> {
        let google_oauth_to_user = self.google_oauth_to_user.lock().await;
        Ok(google_oauth_to_user.get(google_oauth).copied())
    }

    async fn get_github_oauth2_url(&self, oauth_action: &str) -> anyhow::Result<String> {
        match oauth_action {
            "login" | "signup" | "bind" => Ok(format!(
                "http://mock-github-oauth.example.com/auth?action={}",
                oauth_action
            )),
            _ => Err(anyhow::anyhow!("Invalid OAuth action")),
        }
    }

    async fn get_github_oauth_by_authorize_code(
        &self,
        code: &str,
        oauth_action: &str,
    ) -> anyhow::Result<String> {
        if oauth_action != "login" && oauth_action != "signup" && oauth_action != "bind" {
            return Err(anyhow::anyhow!("Invalid OAuth action"));
        }
        // In mock mode, just return a mock OAuth ID based on the code
        Ok(format!("github_oauth_{}", code))
    }

    async fn save_user_github_oauth(&self, id: UserId, github_oauth: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        self.ensure_user_exists(&mut users, id);
        users.get_mut(&id).unwrap().github_oauth = Some(github_oauth.to_string());

        let mut github_oauth_to_user = self.github_oauth_to_user.lock().await;
        github_oauth_to_user.insert(github_oauth.to_string(), id);

        Ok(())
    }

    async fn update_user_github_oauth(&self, id: UserId, github_oauth: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            // Remove old OAuth mapping if it exists
            if let Some(old_oauth) = &auth.github_oauth {
                let mut github_oauth_to_user = self.github_oauth_to_user.lock().await;
                github_oauth_to_user.remove(old_oauth);
            }

            auth.github_oauth = Some(github_oauth.to_string());

            let mut github_oauth_to_user = self.github_oauth_to_user.lock().await;
            github_oauth_to_user.insert(github_oauth.to_string(), id);

            Ok(())
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    async fn verify_user_github_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let users = self.users.lock().await;
        Ok(users
            .get(&id)
            .and_then(|auth| auth.github_oauth.as_ref())
            .is_some())
    }

    async fn delete_user_github_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            if let Some(oauth) = auth.github_oauth.take() {
                let mut github_oauth_to_user = self.github_oauth_to_user.lock().await;
                github_oauth_to_user.remove(&oauth);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn get_user_id_by_github_oauth(
        &self,
        github_oauth: &str,
    ) -> anyhow::Result<Option<UserId>> {
        let github_oauth_to_user = self.github_oauth_to_user.lock().await;
        Ok(github_oauth_to_user.get(github_oauth).copied())
    }

    async fn save_user_traq_oauth(&self, id: UserId, traq_oauth: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        self.ensure_user_exists(&mut users, id);
        users.get_mut(&id).unwrap().traq_oauth = Some(traq_oauth.to_string());

        let mut traq_oauth_to_user = self.traq_oauth_to_user.lock().await;
        traq_oauth_to_user.insert(traq_oauth.to_string(), id);

        Ok(())
    }

    async fn update_user_traq_oauth(&self, id: UserId, traq_oauth: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            // Remove old OAuth mapping if it exists
            if let Some(old_oauth) = &auth.traq_oauth {
                let mut traq_oauth_to_user = self.traq_oauth_to_user.lock().await;
                traq_oauth_to_user.remove(old_oauth);
            }

            auth.traq_oauth = Some(traq_oauth.to_string());

            let mut traq_oauth_to_user = self.traq_oauth_to_user.lock().await;
            traq_oauth_to_user.insert(traq_oauth.to_string(), id);

            Ok(())
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    async fn verify_user_traq_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let users = self.users.lock().await;
        Ok(users
            .get(&id)
            .and_then(|auth| auth.traq_oauth.as_ref())
            .is_some())
    }

    async fn delete_user_traq_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let mut users = self.users.lock().await;
        if let Some(auth) = users.get_mut(&id) {
            if let Some(oauth) = auth.traq_oauth.take() {
                let mut traq_oauth_to_user = self.traq_oauth_to_user.lock().await;
                traq_oauth_to_user.remove(&oauth);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn get_user_id_by_traq_oauth(&self, traq_oauth: &str) -> anyhow::Result<Option<UserId>> {
        let traq_oauth_to_user = self.traq_oauth_to_user.lock().await;
        Ok(traq_oauth_to_user.get(traq_oauth).copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_user_id() -> UserId {
        UserId(Uuid::new_v4())
    }

    #[tokio::test]
    async fn test_password_operations() {
        let mock = AuthRepositoryMock::new();
        let user_id = create_test_user_id();

        // Save email and password
        mock.save_user_email_and_password(user_id, "test@example.com", "password123")
            .await
            .unwrap();

        // Verify password
        assert!(
            mock.verify_user_password(user_id, "password123")
                .await
                .unwrap()
        );
        assert!(!mock.verify_user_password(user_id, "wrong").await.unwrap());

        // Update password
        mock.update_user_password(user_id, "newpass456")
            .await
            .unwrap();
        assert!(
            mock.verify_user_password(user_id, "newpass456")
                .await
                .unwrap()
        );
        assert!(
            !mock
                .verify_user_password(user_id, "password123")
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_google_oauth_operations() {
        let mock = AuthRepositoryMock::new();
        let user_id = create_test_user_id();

        // Get OAuth URL
        let url = mock.get_google_oauth2_url("login").await.unwrap();
        assert!(url.contains("mock-google-oauth"));

        // Get OAuth by code
        let oauth_id = mock
            .get_google_oauth_by_authorize_code("testcode", "login")
            .await
            .unwrap();
        assert_eq!(oauth_id, "google_oauth_testcode");

        // Save OAuth
        mock.save_user_google_oauth(user_id, &oauth_id)
            .await
            .unwrap();

        // Verify OAuth exists
        assert!(mock.verify_user_google_oauth(user_id).await.unwrap());

        // Get user by OAuth
        let found_user = mock.get_user_id_by_google_oauth(&oauth_id).await.unwrap();
        assert_eq!(found_user, Some(user_id));

        // Delete OAuth
        assert!(mock.delete_user_google_oauth(user_id).await.unwrap());
        assert!(!mock.verify_user_google_oauth(user_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_github_oauth_operations() {
        let mock = AuthRepositoryMock::new();
        let user_id = create_test_user_id();

        let oauth_id = mock
            .get_github_oauth_by_authorize_code("testcode", "signup")
            .await
            .unwrap();
        mock.save_user_github_oauth(user_id, &oauth_id)
            .await
            .unwrap();

        assert!(mock.verify_user_github_oauth(user_id).await.unwrap());

        let found_user = mock.get_user_id_by_github_oauth(&oauth_id).await.unwrap();
        assert_eq!(found_user, Some(user_id));
    }

    #[tokio::test]
    async fn test_traq_oauth_operations() {
        let mock = AuthRepositoryMock::new();
        let user_id = create_test_user_id();

        mock.save_user_traq_oauth(user_id, "traq_oauth_123")
            .await
            .unwrap();

        assert!(mock.verify_user_traq_oauth(user_id).await.unwrap());

        let found_user = mock
            .get_user_id_by_traq_oauth("traq_oauth_123")
            .await
            .unwrap();
        assert_eq!(found_user, Some(user_id));

        assert!(mock.delete_user_traq_oauth(user_id).await.unwrap());
        assert!(!mock.verify_user_traq_oauth(user_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_count_authentication_methods() {
        let mock = AuthRepositoryMock::new();
        let user_id = create_test_user_id();

        // Initially 0
        assert_eq!(mock.count_authentication_methods(user_id).await.unwrap(), 0);

        // Add password
        mock.save_user_email_and_password(user_id, "test@example.com", "pass")
            .await
            .unwrap();
        assert_eq!(mock.count_authentication_methods(user_id).await.unwrap(), 1);

        // Add Google OAuth
        mock.save_user_google_oauth(user_id, "google_oauth")
            .await
            .unwrap();
        assert_eq!(mock.count_authentication_methods(user_id).await.unwrap(), 2);

        // Add GitHub OAuth
        mock.save_user_github_oauth(user_id, "github_oauth")
            .await
            .unwrap();
        assert_eq!(mock.count_authentication_methods(user_id).await.unwrap(), 3);

        // Add traQ OAuth
        mock.save_user_traq_oauth(user_id, "traq_oauth")
            .await
            .unwrap();
        assert_eq!(mock.count_authentication_methods(user_id).await.unwrap(), 4);
    }

    #[tokio::test]
    async fn test_update_oauth_changes_mapping() {
        let mock = AuthRepositoryMock::new();
        let user_id = create_test_user_id();

        // Save initial OAuth
        mock.save_user_google_oauth(user_id, "oauth_1")
            .await
            .unwrap();
        assert_eq!(
            mock.get_user_id_by_google_oauth("oauth_1").await.unwrap(),
            Some(user_id)
        );

        // Update to new OAuth
        mock.update_user_google_oauth(user_id, "oauth_2")
            .await
            .unwrap();

        // Old OAuth should not work
        assert_eq!(
            mock.get_user_id_by_google_oauth("oauth_1").await.unwrap(),
            None
        );

        // New OAuth should work
        assert_eq!(
            mock.get_user_id_by_google_oauth("oauth_2").await.unwrap(),
            Some(user_id)
        );
    }
}
