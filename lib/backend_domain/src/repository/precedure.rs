use axum::async_trait;

use crate::model::precedure::Procedure;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait ProcedureRepository {
    async fn create_procedure(&self, problem_id: i64, procedure: Procedure) -> anyhow::Result<()>;
    async fn update_precedure(&self, problem_id: i64, procedure: Procedure) -> anyhow::Result<()>;
    async fn get_procedure(&self, problem_id: i64) -> anyhow::Result<Option<Procedure>>;
    async fn delete_procedure(&self, problem_id: i64) -> anyhow::Result<()>;
}
