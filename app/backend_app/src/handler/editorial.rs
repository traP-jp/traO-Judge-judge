use crate::{di::DiContainer, model::editorials::EditorialSummaryResponse};
use crate::model::editorials::{CreateEditorial, EditorialResponse, UpdateEditorial};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::{headers::Cookie, TypedHeader};
use reqwest::StatusCode;
use usecase::service::editorial::EditorialError;

pub async fn get_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<i64>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container.editorial_service().get_editorial(session_id, editorial_id).await {
        Ok(editorial) => {
            let resp = EditorialResponse::from(editorial);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            EditorialError::ValidateError => Err(StatusCode::BAD_REQUEST),
            EditorialError::Forbidden => Err(StatusCode::FORBIDDEN),
            EditorialError::NotFound => Err(StatusCode::NOT_FOUND),
            EditorialError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn get_editorials(
    State(di_container): State<DiContainer>,
    Path(problem_id): Path<i64>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .get_editorials(session_id, problem_id)
        .await
    {
        Ok(editorials) => {
            let resp: Vec<EditorialSummaryResponse> = editorials.into_iter().map(|e| e.into()).collect();
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            EditorialError::ValidateError => Err(StatusCode::BAD_REQUEST),
            EditorialError::Forbidden => Err(StatusCode::FORBIDDEN),
            EditorialError::NotFound => Err(StatusCode::NOT_FOUND),
            EditorialError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn post_editorial(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Path(problem_id): Path<i64>,
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
        Err(e) => match e {
            EditorialError::ValidateError => Err(StatusCode::BAD_REQUEST),
            EditorialError::Forbidden => Err(StatusCode::FORBIDDEN),
            EditorialError::NotFound => Err(StatusCode::NOT_FOUND),
            EditorialError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn put_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<i64>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(query): Json<UpdateEditorial>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .put_editorial(session_id, editorial_id, query.into())
        .await
    {
        Ok(editorial) => {
            let resp = EditorialResponse::from(editorial);
            Ok((StatusCode::NO_CONTENT, Json(resp)))
        }
        Err(e) => match e {
            EditorialError::ValidateError => Err(StatusCode::BAD_REQUEST),
            EditorialError::Forbidden => Err(StatusCode::FORBIDDEN),
            EditorialError::NotFound => Err(StatusCode::NOT_FOUND),
            EditorialError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn delete_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<i64>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .editorial_service()
        .delete_editorial(session_id, editorial_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            EditorialError::ValidateError => Err(StatusCode::BAD_REQUEST),
            EditorialError::Forbidden => Err(StatusCode::FORBIDDEN),
            EditorialError::NotFound => Err(StatusCode::NOT_FOUND),
            EditorialError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}