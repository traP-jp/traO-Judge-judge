use crate::di::DiContainer;
use crate::model::editorials::EditorialResponse;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use usecase::service::editorials::EditorialError;

pub async fn get_editorial(
    State(di_container): State<DiContainer>,
    Path(editorial_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .editorial_service()
        .get_editorial(editorial_id)
        .await
    {
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
