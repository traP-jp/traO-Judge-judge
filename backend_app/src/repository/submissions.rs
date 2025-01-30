use sqlx::{types::chrono, FromRow};

use super::Repository;

#[derive(FromRow)]
pub struct Submission {
    pub id: i32,
    pub problem_id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub language_id: i32,
    pub source: String,
    pub judge_status: String,
    pub total_score: i64,
    pub max_time: i32,
    pub max_memory: i32,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(FromRow)]
pub struct Testcase {
    pub submission_id: i32,
    pub testcase_id: i32,
    pub testcase_name: String,
    pub judge_status: String,
    pub score: i64,
    pub time: i32,
    pub memory: i32,
}

impl Repository {
    pub async fn get_submission_by_id(&self, id: i64) -> anyhow::Result<Option<Submission>> {
        let submission = sqlx::query_as::<_, Submission>("SELECT * FROM submissions WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(submission)
    }

    pub async fn get_testcases_by_submission_id(
        &self,
        submission_id: i32,
    ) -> anyhow::Result<Vec<Testcase>> {
        let testcases =
            sqlx::query_as::<_, Testcase>("SELECT * FROM testcases WHERE submission_id = ?")
                .bind(submission_id)
                .fetch_all(&self.pool)
                .await?;

        Ok(testcases)
    }
}
