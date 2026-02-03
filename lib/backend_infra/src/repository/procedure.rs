use axum::{Json, async_trait};
use domain::repository::procedure::ProcedureRepository;
use judge_core::model::procedure::registered::Procedure;
use sqlx::MySqlPool;

use crate::model::procedure::{ProcedureJson, ProcedureRow};

#[derive(Clone)]
pub struct ProcedureRepositoryImpl {
    pool: MySqlPool,
}

impl ProcedureRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProcedureRepository for ProcedureRepositoryImpl {
    async fn create_procedure(&self, problem_id: i64, procedure: Procedure) -> anyhow::Result<()> {
        let procedure = ProcedureJson::from(procedure);

        sqlx::query!(
            r#"
            INSERT INTO 
                `procedures` (
                    `problem_id`, 
                    `procedure`
                ) 
            VALUES 
                (?, ?)
            "#,
            problem_id,
            sqlx::types::Json(procedure)
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_procedure(&self, problem_id: i64, procedure: Procedure) -> anyhow::Result<()> {
        let procedure = ProcedureJson::from(procedure);

        sqlx::query!(
            r#"
            UPDATE 
                `procedures` 
            SET 
                `procedure` = ? 
            WHERE 
                `problem_id` = ?"#,
            sqlx::types::Json(procedure),
            problem_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_procedure(&self, problem_id: i64) -> anyhow::Result<Option<Procedure>> {
        let procedure_row = sqlx::query_as_unchecked!(
            ProcedureRow,
            r#"
            SELECT 
                `procedure` 
            FROM 
                `procedures` 
            WHERE 
                `problem_id` = ?
            "#,
            problem_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(procedure_row.map(|row| row.procedure.0.into()))
    }

    async fn delete_procedure(&self, problem_id: i64) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM 
                `procedures` 
            WHERE 
                `problem_id` = ?
            "#,
            problem_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
