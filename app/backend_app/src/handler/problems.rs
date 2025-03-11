use crate::di::DiContainer;
use crate::model::problems::{CreateNormalProblem, ProblemResponse, UpdateNormalProblem};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::{headers::Cookie, TypedHeader};
use reqwest::StatusCode;
use usecase::model::problem::{CreateNormalProblemData, UpdateNormalProblemData};
use usecase::service::problem::ProblemError;

pub async fn get_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").map(|s| s.to_string());

    match di_container
        .problem_service()
        .get_problem(session_id, problem_id)
        .await
    {
        Ok(problem) => {
            let resp = ProblemResponse::from(problem);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            ProblemError::ValidateError => Err(StatusCode::BAD_REQUEST),
            ProblemError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            ProblemError::Forbidden => Err(StatusCode::FORBIDDEN),
            ProblemError::NotFound => Err(StatusCode::NOT_FOUND),
            ProblemError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn put_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<i64>,
    Json(body): Json<UpdateNormalProblem>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container
        .problem_service()
        .update_problem(
            session_id,
            problem_id,
            UpdateNormalProblemData {
                title: body.title,
                statement: body.statement,
                difficulty: body.difficulty,
                time_limit: body.time_limit,
                memory_limit: body.memory_limit,
                is_public: body.is_public,
            },
        )
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            ProblemError::ValidateError => Err(StatusCode::BAD_REQUEST),
            ProblemError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            ProblemError::Forbidden => Err(StatusCode::FORBIDDEN),
            ProblemError::NotFound => Err(StatusCode::NOT_FOUND),
            ProblemError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn post_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(body): Json<CreateNormalProblem>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container
        .problem_service()
        .create_problem(
            session_id,
            CreateNormalProblemData {
                title: body.title,
                statement: body.statement,
                difficulty: body.difficulty,
                time_limit: body.time_limit,
                memory_limit: body.memory_limit,
            },
        )
        .await
    {
        Ok(problem) => {
            let resp = ProblemResponse::from(problem);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            ProblemError::ValidateError => Err(StatusCode::BAD_REQUEST),
            ProblemError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            ProblemError::Forbidden => Err(StatusCode::FORBIDDEN),
            ProblemError::NotFound => Err(StatusCode::NOT_FOUND),
            ProblemError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
