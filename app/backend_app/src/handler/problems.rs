use crate::di::DiContainer;
use crate::model::error::AppError;
use crate::model::problems::{
    CreateNormalProblem, ProblemGetQuery, ProblemOrderBy, ProblemResponse,
    ProblemSummariesResponses, UpdateNormalProblem,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::Cookie};
use usecase::model::problem::{
    CreateNormalProblemData, ProblemGetQueryData, ProblemOrderByData, UpdateNormalProblemData,
};

pub async fn get_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<String>,
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
        Err(e) => Err(AppError(e).into()),
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
                user_name: query.user_name,
                user_query: query.user_id,
            },
        )
        .await
    {
        Ok(problems) => {
            let resp = ProblemSummariesResponses::from(problems);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn put_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<String>,
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
                time_limit_ms: body.time_limit,
                memory_limit_mib: body.memory_limit,
                is_public: body.is_public,
            },
        )
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
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
                time_limit_ms: body.time_limit,
                memory_limit_mib: body.memory_limit,
            },
        )
        .await
    {
        Ok(problem) => {
            let resp = ProblemResponse::from(problem);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn delete_problem(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .problem_service()
        .delete_problem(session_id, problem_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
