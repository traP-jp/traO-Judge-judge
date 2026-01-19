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
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .get_editorial(session_id, editorial_id)
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
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .get_editorials(session_id, problem_id)
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
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<String>,
    Json(query): Json<CreateEditorial>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .post_editorial(session_id, problem_id, query.into())
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
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(query): Json<UpdateEditorial>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .put_editorial(session_id, editorial_id, query.into())
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn delete_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .delete_editorial(session_id, editorial_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
