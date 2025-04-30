use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
};
use reqwest::StatusCode;
use usecase::service::icon::IconServiceError;

use crate::di::DiContainer;

pub async fn get_icon(
    State(di_container): State<DiContainer>,
    Path(icon_uuid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container.icon_service().get_icon(icon_uuid).await {
        Ok(icon) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", icon.content_type.parse().unwrap());
            Ok((headers, icon.icon))
        }
        Err(e) => match e {
            IconServiceError::NotFound => Err(StatusCode::NOT_FOUND),
            IconServiceError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
