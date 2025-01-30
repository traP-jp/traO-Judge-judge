use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::{headers::Cookie, TypedHeader};
use reqwest::StatusCode;
use serde::Serialize;
use sqlx::types::chrono;

use super::Repository;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubmissionResponse {
    id: String,
    user_id: i32,
    user_name: String,
    problem_id: i32,
    submitted_at: chrono::DateTime<chrono::Utc>,
    language_id: i32,
    total_score: i64,
    max_time: i32,
    max_memory: i32,
    code_length: i32,
    overall_judge_status: String,
    judge_results: Vec<TestcaseResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TestcaseResponse {
    testcase_id: i32,
    testcase_name: String,
    judge_status: String,
    score: i64,
    time: i32,
    memory: i32,
}

pub async fn get_submission(
    State(state): State<Repository>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(path): Path<i64>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let submission = state
        .get_submission_by_id(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let problem = state
        .get_normal_problem_by_id(submission.problem_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !problem.is_public {
        let session_id = cookie.get("session_id").ok_or(StatusCode::NOT_FOUND)?;

        let display_id = state
            .get_display_id_by_session_id(session_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

        if display_id != problem.author_id {
            return Err(StatusCode::NOT_FOUND);
        }
    }

    let testcases = state
        .get_testcases_by_submission_id(submission.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = SubmissionResponse {
        id: submission.id.to_string(),
        user_id: submission.user_id,
        user_name: submission.user_name,
        problem_id: submission.problem_id,
        submitted_at: submission.submitted_at,
        language_id: submission.language_id,
        total_score: submission.total_score,
        max_time: submission.max_time,
        max_memory: submission.max_memory,
        code_length: submission.source.len() as i32,
        overall_judge_status: submission.judge_status,
        judge_results: testcases
            .into_iter()
            .map(|testcase| TestcaseResponse {
                testcase_id: testcase.testcase_id,
                testcase_name: testcase.testcase_name,
                judge_status: testcase.judge_status,
                score: testcase.score,
                time: testcase.time,
                memory: testcase.memory,
            })
            .collect(),
    };

    Ok(Json(response))
}
