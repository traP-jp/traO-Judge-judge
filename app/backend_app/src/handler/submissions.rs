use crate::model::submissions::{
    CreateSubmission, SubmissionOrderBy, SubmissionResponse, SubmissionSummariesResponse,
};
use crate::{di::DiContainer, model::submissions::SubmissionGetQuery};
use axum::extract::Query;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::Cookie};
use reqwest::StatusCode;
use usecase::model::submission::{
    CreateSubmissionData, SubmissionGetQueryData, SubmissionOrderByData,
};
use usecase::service::submission::SubmissionError;

pub async fn get_submission(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(submission_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .submission_service()
        .get_submission(session_id, submission_id)
        .await
    {
        Ok(user) => {
            let resp = SubmissionResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            SubmissionError::ValidateError => Err(StatusCode::BAD_REQUEST),
            SubmissionError::Forbidden => Err(StatusCode::FORBIDDEN),
            SubmissionError::NotFound => Err(StatusCode::NOT_FOUND),
            SubmissionError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn get_submissions(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Query(query): Query<SubmissionGetQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    let query = SubmissionGetQueryData {
        limit: query.limit,
        offset: query.offset,
        judge_status: query.judge_status,
        problem_id: query.problem_id,
        language_id: query.language_id,
        user_name: query.user_name,
        user_query: query.user_id,
        order_by: match query.order_by {
            Some(order_by) => match order_by {
                SubmissionOrderBy::SubmittedAtAsc => SubmissionOrderByData::SubmittedAtAsc,
                SubmissionOrderBy::SubmittedAtDesc => SubmissionOrderByData::SubmittedAtDesc,
                SubmissionOrderBy::TimeConsumptionAsc => SubmissionOrderByData::TimeConsumptionAsc,
                SubmissionOrderBy::TimeConsumptionDesc => {
                    SubmissionOrderByData::TimeConsumptionDesc
                }
                SubmissionOrderBy::ScoreAsc => SubmissionOrderByData::ScoreAsc,
                SubmissionOrderBy::ScoreDesc => SubmissionOrderByData::ScoreDesc,
                SubmissionOrderBy::MemoryConsumptionAsc => {
                    SubmissionOrderByData::MemoryConsumptionAsc
                }
                SubmissionOrderBy::MemoryConsumptionDesc => {
                    SubmissionOrderByData::MemoryConsumptionDesc
                }
                SubmissionOrderBy::CodeLengthAsc => SubmissionOrderByData::CodeLengthAsc,
                SubmissionOrderBy::CodeLengthDesc => SubmissionOrderByData::CodeLengthDesc,
            },
            None => SubmissionOrderByData::SubmittedAtDesc,
        },
    };

    match di_container
        .submission_service()
        .get_submissions(session_id, query)
        .await
    {
        Ok(submissions) => {
            let resp = SubmissionSummariesResponse::from(submissions);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            SubmissionError::ValidateError => Err(StatusCode::BAD_REQUEST),
            SubmissionError::Forbidden => Err(StatusCode::FORBIDDEN),
            SubmissionError::NotFound => Err(StatusCode::NOT_FOUND),
            SubmissionError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn post_submission(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<String>,
    Json(body): Json<CreateSubmission>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .submission_service()
        .create_submission(
            session_id,
            problem_id,
            CreateSubmissionData {
                language_id: body.language_id,
                source: body.source,
            },
        )
        .await
    {
        Ok(submission) => {
            let resp = SubmissionResponse::from(submission);
            Ok((StatusCode::CREATED, Json(resp)))
        }
        Err(e) => match e {
            SubmissionError::ValidateError => Err(StatusCode::BAD_REQUEST),
            SubmissionError::Forbidden => Err(StatusCode::FORBIDDEN),
            SubmissionError::NotFound => Err(StatusCode::NOT_FOUND),
            SubmissionError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
