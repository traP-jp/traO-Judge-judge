use anyhow::Context;
use async_session::{Session, SessionStore};
use async_sqlx_session::MySqlSessionStore;
use axum::async_trait;
use domain::{
    model::session::SessionUser,
    model::user::{User, UserDisplayId, UserId},
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
            .insert(
                "display_id",
                <UserDisplayId as Into<i64>>::into(user.display_id),
            )
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

    async fn get_session_user(&self, session_id: &str) -> anyhow::Result<Option<SessionUser>> {
        let session = self
            .session_store
            .load_session(session_id.to_string())
            .await?;

        if let Some(session) = &session {
            let user_id_str: String = session
                .get("user_id")
                .with_context(|| "Failed to get user_id from session")?;
            let user_id_uuid = uuid::Uuid::parse_str(&user_id_str)
                .with_context(|| "Failed to parse user_id from session")?;
            let display_id_i64: i64 = session
                .get("display_id")
                .with_context(|| "Failed to get display_id from session")?;
            let user_id = UserId::from(user_id_uuid);
            let display_id = UserDisplayId::from(display_id_i64);

            Ok(Some(SessionUser {
                user_id,
                display_id,
            }))
        } else {
            Ok(None)
        }
    }
}
