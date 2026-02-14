use crate::extractor::session::ExtractedSessionUser;
use crate::model::error::AppError;
use crate::model::submissions::{
    CreateSubmission, SubmissionResponse, SubmissionSummariesResponse,
};
use crate::{di::DiContainer, model::submissions::SubmissionGetQuery};
use axum::extract::Query;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use usecase::model::submission::CreateSubmissionData;

pub async fn get_submission(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Path(submission_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let submission_id = uuid::Uuid::parse_str(&submission_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();
    match di_container
        .submission_service()
        .get_submission(session_user, submission_id)
        .await
    {
        Ok(user) => {
            let resp = SubmissionResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn get_submissions(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Query(query): Query<SubmissionGetQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let query = query.try_into()?;

    match di_container
        .submission_service()
        .get_submissions(session_user, query)
        .await
    {
        Ok(submissions) => {
            let resp = SubmissionSummariesResponse::from(submissions);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn post_submission(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Path(problem_id): Path<String>,
    Json(body): Json<CreateSubmission>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .submission_service()
        .create_submission(
            session_user,
            problem_id
                .parse::<i64>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
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
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn post_rejudge_submission(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Path(submission_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .submission_service()
        .rejudge_submission(
            session_user,
            uuid::Uuid::parse_str(&submission_id)
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
        )
        .await
    {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
