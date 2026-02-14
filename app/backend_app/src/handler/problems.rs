use crate::di::DiContainer;
use crate::extractor::session::ExtractedSessionUser;
use crate::model::error::AppError;
use crate::model::problems::{
    CreateNormalProblem, ProblemGetQuery, ProblemOrderBy, ProblemResponse,
    ProblemSummariesResponses, UpdateNormalProblem,
};
use async_session::Session;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use usecase::model::problem::{
    CreateNormalProblemData, ProblemGetQueryData, ProblemOrderByData, UpdateNormalProblemData,
};

pub async fn get_problem(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Path(problem_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let problem_id = problem_id
        .parse::<i64>()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();
    match di_container
        .problem_service()
        .get_problem(session_user, problem_id)
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
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Query(query): Query<ProblemGetQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .problem_service()
        .get_problems_by_query(
            session_user,
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
                user_name: query.username,
                user_query: query
                    .user_id
                    .map(|user_id| {
                        user_id
                            .parse::<i64>()
                            .map_err(|_| StatusCode::BAD_REQUEST)
                            .map(|user_id| user_id.into())
                    })
                    .transpose()?,
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
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Path(problem_id): Path<String>,
    Json(body): Json<UpdateNormalProblem>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .problem_service()
        .update_problem(
            session_user,
            problem_id
                .parse::<i64>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
            UpdateNormalProblemData {
                title: body.title,
                statement: body.statement,
                difficulty: body.difficulty,
                time_limit_ms: body.time_limit,
                memory_limit_kib: body.memory_limit,
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
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Json(body): Json<CreateNormalProblem>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .problem_service()
        .create_problem(
            session_user,
            CreateNormalProblemData {
                title: body.title,
                statement: body.statement,
                difficulty: body.difficulty,
                time_limit_ms: body.time_limit,
                memory_limit_kib: body.memory_limit,
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
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Path(problem_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .problem_service()
        .delete_problem(
            session_user,
            problem_id
                .parse::<i64>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
        )
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
