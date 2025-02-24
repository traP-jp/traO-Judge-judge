use crate::model::submissions::SubmissionResponse;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::{headers::Cookie, TypedHeader};
use di::DiContainer;
use reqwest::StatusCode;
use usecase::service::submission::SubmissionError;

pub async fn get_submission(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(submission_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").map(|s| s.to_string());

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
