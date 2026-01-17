use crate::model::{
    submission::{JudgeResultRow, SubmissionRow},
    uuid::UuidRow,
};
use axum::async_trait;
use domain::{
    model::submission::{
        CreateJudgeResult, CreateSubmission, JudgeResult, Submission, SubmissionGetQuery,
        SubmissionOrderBy, UpdateSubmission,
    },
    repository::submission::SubmissionRepository,
};
use sqlx::{MySqlPool, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct SubmissionRepositoryImpl {
    pool: MySqlPool,
}

impl SubmissionRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryImpl {
    async fn get_submission(&self, id: Uuid) -> anyhow::Result<Option<Submission>> {
        let submission = sqlx::query_as::<_, SubmissionRow>(
            "SELECT submissions.*, normal_problems.title as problem_title, users.name as user_name FROM submissions INNER JOIN normal_problems ON normal_problems.id = submissions.problem_id LEFT JOIN users ON users.display_id = submissions.user_id WHERE submissions.id = ?"
        )
        .bind(UuidRow(id))
        .fetch_optional(&self.pool)
        .await?;

        Ok(submission.map(|submission| submission.into()))
    }

    async fn get_submission_results(&self, id: Uuid) -> anyhow::Result<Vec<JudgeResult>> {
        let results = sqlx::query_as::<_, JudgeResultRow>(
            "SELECT * FROM submission_testcases WHERE submission_id = ?",
        )
        .bind(UuidRow(id))
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(|result| result.into()).collect())
    }

    async fn get_submissions_by_query(
        &self,
        query: SubmissionGetQuery,
    ) -> anyhow::Result<Vec<Submission>> {
        let mut query_builder = QueryBuilder::new(
            "SELECT submissions.*, normal_problems.title as problem_title, users.name as user_name FROM submissions INNER JOIN normal_problems ON normal_problems.id = submissions.problem_id LEFT JOIN users ON users.display_id = submissions.user_id WHERE",
        );

        query_builder.push(" (normal_problems.is_public = TRUE");
        if let Some(user_id) = query.user_id {
            query_builder
                .push(" OR normal_problems.author_id = ")
                .push_bind(user_id);
        }
        query_builder.push(")");

        if let Some(user_query) = query.user_query {
            query_builder
                .push(" AND submissions.user_id = ")
                .push_bind(user_query);
        }
        if let Some(user_name) = query.user_name {
            query_builder
                .push(" AND users.name = ")
                .push_bind(user_name);
        }
        if let Some(language_id) = query.language_id {
            query_builder
                .push(" AND submissions.language_id = ")
                .push_bind(language_id);
        }
        if let Some(judge_status) = query.judge_status {
            query_builder
                .push(" AND submissions.judge_status = ")
                .push_bind(judge_status);
        }
        if let Some(problem_id) = query.problem_id {
            query_builder
                .push(" AND submissions.problem_id = ")
                .push_bind(problem_id);
        }

        query_builder.push(" ORDER BY ");

        match query.order_by {
            SubmissionOrderBy::SubmittedAtAsc => {
                query_builder.push("submissions.submitted_at ASC");
            }
            SubmissionOrderBy::SubmittedAtDesc => {
                query_builder.push("submissions.submitted_at DESC");
            }
            SubmissionOrderBy::TimeConsumptionAsc => {
                query_builder.push("submissions.max_time_ms ASC");
            }
            SubmissionOrderBy::TimeConsumptionDesc => {
                query_builder.push("submissions.max_time_ms DESC");
            }
            SubmissionOrderBy::ScoreAsc => {
                query_builder.push("submissions.total_score ASC");
            }
            SubmissionOrderBy::ScoreDesc => {
                query_builder.push("submissions.total_score DESC");
            }
            SubmissionOrderBy::MemoryConsumptionAsc => {
                query_builder.push("submissions.max_memory_kib ASC");
            }
            SubmissionOrderBy::MemoryConsumptionDesc => {
                query_builder.push("submissions.max_memory_kib DESC");
            }
            SubmissionOrderBy::CodeLengthAsc => {
                query_builder.push("LENGTH(submissions.source) ASC");
            }
            SubmissionOrderBy::CodeLengthDesc => {
                query_builder.push("LENGTH(submissions.source) DESC");
            }
        }

        query_builder.push(" LIMIT ").push_bind(query.limit);
        query_builder.push(" OFFSET ").push_bind(query.offset);

        let submissions = query_builder
            .build_query_as::<SubmissionRow>()
            .fetch_all(&self.pool)
            .await?;

        Ok(submissions
            .into_iter()
            .map(|submission| submission.into())
            .collect())
    }

    async fn get_submissions_count_by_query(
        &self,
        query: SubmissionGetQuery,
    ) -> anyhow::Result<i64> {
        let mut query_builder = QueryBuilder::new(
            "SELECT COUNT(*) FROM submissions INNER JOIN normal_problems ON normal_problems.id = submissions.problem_id LEFT JOIN users ON users.display_id = submissions.user_id \nWHERE",
        );

        query_builder.push(" (normal_problems.is_public = TRUE");
        if let Some(user_id) = query.user_id {
            query_builder
                .push(" OR normal_problems.author_id = ")
                .push_bind(user_id);
        }
        query_builder.push(")");

        if let Some(user_query) = query.user_query {
            query_builder
                .push(" AND submissions.user_id = ")
                .push_bind(user_query);
        }
        if let Some(user_name) = query.user_name {
            query_builder
                .push(" AND users.name = ")
                .push_bind(user_name);
        }
        if let Some(language_id) = query.language_id {
            query_builder
                .push(" AND submissions.language_id = ")
                .push_bind(language_id);
        }
        if let Some(judge_status) = query.judge_status {
            query_builder
                .push(" AND submissions.judge_status = ")
                .push_bind(judge_status);
        }
        if let Some(problem_id) = query.problem_id {
            query_builder
                .push(" AND submissions.problem_id = ")
                .push_bind(problem_id);
        }

        let count = query_builder
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    async fn create_submission(&self, submission: CreateSubmission) -> anyhow::Result<Uuid> {
        let submission_id = Uuid::now_v7();

        sqlx::query(
            "INSERT INTO submissions (id, problem_id, user_id, language_id, source, judge_status, total_score, max_time_ms, max_memory_kib) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(UuidRow(submission_id))
        .bind(submission.problem_id)
        .bind(submission.user_id)
        .bind(submission.language_id)
        .bind(submission.source)
        .bind(submission.judge_status)
        .bind(submission.total_score)
        .bind(submission.max_time_ms)
        .bind(submission.max_memory_kib)
        .execute(&self.pool)
        .await?;

        Ok(submission_id)
    }

    async fn update_submission(
        &self,
        submission_id: Uuid,
        submission: UpdateSubmission,
    ) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE submissions SET judge_status = ?, total_score = ?, max_time_ms = ?, max_memory_kib = ? WHERE id = ?",
        )
        .bind(submission.judge_status)
        .bind(submission.total_score)
        .bind(submission.max_time_ms)
        .bind(submission.max_memory_kib)
        .bind(UuidRow(submission_id))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_judge_results(&self, results: Vec<CreateJudgeResult>) -> anyhow::Result<()> {
        if results.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO submission_testcases (submission_id, testcase_id, testcase_name, judge_status, score, time_ms, memory_kib) VALUES ",
        );
        let mut separated = query_builder.separated(", ");
        for r in results.into_iter() {
            separated.push("(");
            separated.push_bind_unseparated(UuidRow(r.submission_id));
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(UuidRow(r.testcase_id));
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(r.testcase_name);
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(r.judge_status);
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(r.score);
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(r.time_ms);
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(r.memory_kib);
            separated.push_unseparated(")");
        }
        query_builder.build().execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_judge_results_by_submission_id(
        &self,
        submission_id: Uuid,
    ) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM submission_testcases WHERE submission_id = ?")
            .bind(UuidRow(submission_id))
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
