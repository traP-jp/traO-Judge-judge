use axum::async_trait;
use domain::{
    model::icon::{CreateIcon, Icon, IconId},
    repository::icon::IconRepository,
};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::model::{icon::IconRow, uuid::UuidRow};

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
    async fn get_icon(&self, id: IconId) -> anyhow::Result<Option<Icon>> {
        let icon = sqlx::query_as!(
            IconRow,
            r#"
            SELECT
                id AS "id: _",
                content_type,
                icon
            FROM 
                icons 
            WHERE 
                id = ?
            "#,
            UuidRow(id.into())
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(icon.map(|row| row.into()))
    }

    async fn create_icon(&self, icon: CreateIcon) -> anyhow::Result<IconId> {
        let uuid = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO 
                icons (
                    id, 
                    content_type, 
                    icon
                ) 
            VALUES 
                (?, ?, ?)
            "#,
            UuidRow(uuid),
            icon.content_type,
            &icon.icon
        )
        .execute(&self.pool)
        .await?;

        Ok(uuid.into())
    }

    async fn delete_icon(&self, id: IconId) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM 
                icons 
            WHERE 
                id = ?
            "#,
            UuidRow(id.into())
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
