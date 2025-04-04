use axum::async_trait;
use domain::{model::precedure::Procedure, repository::precedure::ProcedureRepository};
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

        sqlx::query("INSERT INTO `procedures` (`problem_id`, `procedure`) VALUES (?, ?)")
            .bind(problem_id)
            .bind(sqlx::types::Json(procedure))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_precedure(&self, problem_id: i64, procedure: Procedure) -> anyhow::Result<()> {
        let procedure = ProcedureJson::from(procedure);

        sqlx::query("UPDATE `procedures` SET `procedure` = ? WHERE `problem_id` = ?")
            .bind(sqlx::types::Json(procedure))
            .bind(problem_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_procedure(&self, problem_id: i64) -> anyhow::Result<Option<Procedure>> {
        let procedure_row = sqlx::query_as::<_, ProcedureRow>(
            "SELECT `procedure` FROM `procedures` WHERE `problem_id` = ?",
        )
        .bind(problem_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(procedure_row.map(|row| row.procedure.0.into()))
    }

    async fn delete_procedure(&self, problem_id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM `procedures` WHERE `problem_id` = ?")
            .bind(problem_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
