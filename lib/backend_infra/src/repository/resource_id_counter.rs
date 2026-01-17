use axum::async_trait;
use domain::repository::resource_id_counter::ResourceIdCounterRepository;
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct ResourceIdRow {
    resource_id: String,
}

#[derive(Clone)]
pub struct ResourceIdCounterRepositoryImpl {
    pool: MySqlPool,
}

impl ResourceIdCounterRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResourceIdCounterRepository for ResourceIdCounterRepositoryImpl {
    async fn get_deletable_resource_ids(&self, limit: usize) -> anyhow::Result<Vec<Uuid>> {
        let rows = sqlx::query_as::<_, ResourceIdRow>(
            r#"
            SELECT resource_id
            FROM resource_id_counter
            WHERE ref_count = 0 AND updated_at < NOW() - INTERVAL 1 HOUR
            ORDER BY updated_at ASC
            LIMIT ?
            "#,
        )
        .bind(limit as u32)
        .fetch_all(&self.pool)
        .await?;

        let uuids = rows
            .into_iter()
            .filter_map(|row| Uuid::parse_str(&row.resource_id).ok())
            .collect();

        Ok(uuids)
    }

    async fn delete_resource_ids(&self, ids: Vec<Uuid>) -> anyhow::Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let mut query_builder =
            sqlx::QueryBuilder::new("DELETE FROM resource_id_counter WHERE resource_id IN (");

        for (i, id) in ids.iter().enumerate() {
            if i > 0 {
                query_builder.push(", ");
            }
            query_builder.push_bind(id.to_string());
        }
        query_builder.push(")");

        let query = query_builder.build();
        query.execute(&self.pool).await?;

        Ok(())
    }

    async fn update_timestamp_ids(&self, ids: Vec<Uuid>) -> anyhow::Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "UPDATE resource_id_counter SET updated_at = NOW() WHERE resource_id IN (",
        );

        for (i, id) in ids.iter().enumerate() {
            if i > 0 {
                query_builder.push(", ");
            }
            query_builder.push_bind(id.to_string());
        }
        query_builder.push(")");

        let query = query_builder.build();
        query.execute(&self.pool).await?;

        Ok(())
    }
}
