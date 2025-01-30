use super::Repository;
use sqlx::types::chrono;

#[derive(sqlx::FromRow)]
pub struct NormalProblems {
    pub id: i32,
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub judgecode_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Repository {
    pub async fn get_normal_problem_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<Option<NormalProblems>> {
        let problem =
            sqlx::query_as::<_, NormalProblems>("SELECT * FROM normal_problems WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(problem)
    }
}
