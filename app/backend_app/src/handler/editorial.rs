use crate::extractor::session::ExtractedSessionUser;
use crate::model::editorials::{CreateEditorial, EditorialResponse, UpdateEditorial};
use crate::model::error::AppError;
use crate::{di::DiContainer, model::editorials::EditorialSummaryResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::Cookie};

pub async fn get_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .editorial_service()
        .get_editorial(
            session_user,
            uuid::Uuid::parse_str(&editorial_id)
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
        )
        .await
    {
        Ok(editorial) => {
            let resp = EditorialResponse::from(editorial);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn get_editorials(
    State(di_container): State<DiContainer>,
    Path(problem_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .editorial_service()
        .get_editorials(
            session_user,
            problem_id
                .parse::<i64>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
        )
        .await
    {
        Ok(editorials) => {
            let resp: Vec<EditorialSummaryResponse> =
                editorials.into_iter().map(|e| e.into()).collect();
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn post_editorial(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Path(problem_id): Path<String>,
    Json(query): Json<CreateEditorial>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .editorial_service()
        .post_editorial(
            session_user,
            problem_id
                .parse::<i64>()
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
            query.into(),
        )
        .await
    {
        Ok(editorial) => {
            let resp = EditorialResponse::from(editorial);
            Ok((StatusCode::CREATED, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn put_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Json(query): Json<UpdateEditorial>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .editorial_service()
        .put_editorial(
            session_user,
            uuid::Uuid::parse_str(&editorial_id)
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
            query.into(),
        )
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn delete_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<String>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .editorial_service()
        .delete_editorial(
            session_user,
            uuid::Uuid::parse_str(&editorial_id)
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .into(),
        )
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
