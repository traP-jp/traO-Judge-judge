use axum::async_trait;
use domain::{
    model::user::UserId,
    repository::auth::AuthRepository,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
struct UserAuth {
    password: Option<String>,
    google_oauth: Option<String>,
    github_oauth: Option<String>,
    traq_oauth: Option<String>,
}

/// Mock implementation of AuthRepository for testing and development
/// Stores authentication data in memory instead of a database.
#[derive(Clone)]
pub struct AuthRepositoryMock {
    users: Arc<Mutex<HashMap<UserId, UserAuth>>>,
    google_oauth_to_user: Arc<Mutex<HashMap<String, UserId>>>,
    github_oauth_to_user: Arc<Mutex<HashMap<String, UserId>>>,
    traq_oauth_to_user: Arc<Mutex<HashMap<String, UserId>>>,
}

impl AuthRepositoryMock {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            google_oauth_to_user: Arc::new(Mutex::new(HashMap::new())),
            github_oauth_to_user: Arc::new(Mutex::new(HashMap::new())),
            traq_oauth_to_user: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn ensure_user_exists(&self, users: &mut HashMap<UserId, UserAuth>, id: UserId) {
        users.entry(id).or_insert_with(|| UserAuth {
            password: None,
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

    async fn save_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let mut users = self.users.lock().await;
        self.ensure_user_exists(&mut users, id);
        users.get_mut(&id).unwrap().password = Some(password.to_string());
        Ok(())
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
            "login" | "signup" | "bind" => {
                Ok(format!("http://mock-google-oauth.example.com/auth?action={}", oauth_action))
            }
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
        Ok(users.get(&id).and_then(|auth| auth.google_oauth.as_ref()).is_some())
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
            "login" | "signup" | "bind" => {
                Ok(format!("http://mock-github-oauth.example.com/auth?action={}", oauth_action))
            }
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
        Ok(users.get(&id).and_then(|auth| auth.github_oauth.as_ref()).is_some())
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
        Ok(users.get(&id).and_then(|auth| auth.traq_oauth.as_ref()).is_some())
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
