use axum::async_trait;
use domain::{model::icon::Icon, repository::icon::IconRepository};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::model::icon::IconRow;

#[derive(Clone)]
pub struct IconRepositoryImpl {
    pool: MySqlPool,
}

impl IconRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IconRepository for IconRepositoryImpl {
    async fn get_icon(&self, id: Uuid) -> anyhow::Result<Option<Icon>> {
        let icon = sqlx::query_as::<_, IconRow>("SELECT * FROM icons WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(icon.map(|row| row.into()))
    }

    async fn create_icon(&self, icon: Icon) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO icons (id, content_type, icon) VALUES (?, ?, ?)")
            .bind(icon.id)
            .bind(icon.content_type)
            .bind(&icon.icon)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
