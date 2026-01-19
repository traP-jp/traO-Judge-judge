use anyhow::Context;
use async_session::{Session, SessionStore};
use async_sqlx_session::MySqlSessionStore;
use axum::async_trait;
use domain::{
    model::user::{User, UserId},
    repository::session::SessionRepository,
};

#[derive(Clone)]
pub struct SessionRepositoryImpl {
    session_store: MySqlSessionStore,
}

impl SessionRepositoryImpl {
    pub fn new(session_store: MySqlSessionStore) -> Self {
        Self { session_store }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn create_session(&self, user: User) -> anyhow::Result<String> {
        let mut session = Session::new();
        session
            .insert("user_id", user.id.to_string())
            .with_context(|| "Failed to insert user_id to session")?;
        session
            .insert("display_id", user.display_id)
            .with_context(|| "Failed to insert display_id to session")?;
        let result = self
            .session_store
            .store_session(session)
            .await
            .with_context(|| "Failed to store session to database")
            .with_context(|| "Failed to create session")?;
        match result {
            Some(session_id) => Ok(session_id),
            None => anyhow::bail!("unexpected error while creating session"),
        }
    }

    async fn delete_session(&self, session_id: &str) -> anyhow::Result<Option<()>> {
        let Some(session) = self
            .session_store
            .load_session(session_id.to_string())
            .await?
        else {
            return Ok(None);
        };

        self.session_store.destroy_session(session).await?;
        Ok(Some(()))
    }

    async fn get_user_id_by_session_id(&self, session_id: &str) -> anyhow::Result<Option<UserId>> {
        let session = self
            .session_store
            .load_session(session_id.to_string())
            .await?;

        let user_id = session
            .and_then(|s| s.get("user_id"))
            .map(|id: uuid::Uuid| id.into());

        Ok(user_id)
    }

    async fn get_display_id_by_session_id(&self, session_id: &str) -> anyhow::Result<Option<i64>> {
        let session = self
            .session_store
            .load_session(session_id.to_string())
            .await?;

        Ok(session.and_then(|s| s.get("display_id")))
    }
}
