use crate::di::DiContainer;
use crate::model::problems::{
    CreateNormalProblem, ProblemGetQuery, ProblemOrderBy, ProblemResponse,
    ProblemSummariesResponses, UpdateNormalProblem,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::Cookie};
use reqwest::StatusCode;
use usecase::model::problem::{
    CreateNormalProblemData, ProblemGetQueryData, ProblemOrderByData, UpdateNormalProblemData,
};
use usecase::service::problem::ProblemError;

pub async fn get_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

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

pub async fn get_problems(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Query(query): Query<ProblemGetQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .problem_service()
        .get_problems_by_query(
            session_id,
            ProblemGetQueryData {
                offset: query.offset,
                limit: query.limit,
                order_by: match query.order_by {
                    Some(order_by) => match order_by {
                        ProblemOrderBy::CreatedAtAsc => ProblemOrderByData::CreatedAtAsc,
                        ProblemOrderBy::CreatedAtDesc => ProblemOrderByData::CreatedAtDesc,
                        ProblemOrderBy::UpdatedAtAsc => ProblemOrderByData::UpdatedAtAsc,
                        ProblemOrderBy::UpdatedAtDesc => ProblemOrderByData::UpdatedAtDesc,
                        ProblemOrderBy::DifficultyAsc => ProblemOrderByData::DifficultyAsc,
                        ProblemOrderBy::DifficultyDesc => ProblemOrderByData::DifficultyDesc,
                    },
                    None => ProblemOrderByData::CreatedAtDesc,
                },
                user_query: query.user_id,
            },
        )
        .await
    {
        Ok(problems) => {
            let resp = ProblemSummariesResponses::from(problems);
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
    let session_id = cookie.get("session_id");

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
    let session_id = cookie.get("session_id");

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

pub async fn delete_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .problem_service()
        .delete_problem(session_id, problem_id)
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
