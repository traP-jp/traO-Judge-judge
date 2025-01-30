use super::{users::UserId, Repository};
use anyhow::Context;
use async_session::{Session, SessionStore};

use super::users::User;

impl Repository {
    pub async fn create_session(&self, user: User) -> anyhow::Result<String> {
        let mut session = Session::new();
        session
            .insert("user_id", user.id)
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

    pub async fn delete_session(&self, session_id: &str) -> anyhow::Result<Option<()>> {
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

    pub async fn get_user_id_by_session_id(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Option<UserId>> {
        let session = self
            .session_store
            .load_session(session_id.to_string())
            .await?;

        let user_id = session
            .and_then(|s| s.get("user_id"))
            .map(|id: uuid::Uuid| UserId::new(id));

        Ok(user_id)
    }

    pub async fn get_display_id_by_session_id(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Option<i64>> {
        let session = self
            .session_store
            .load_session(session_id.to_string())
            .await?;

        Ok(session.and_then(|s| s.get("display_id")))
    }
}
