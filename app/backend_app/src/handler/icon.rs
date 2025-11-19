use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::{di::DiContainer, model::error::AppError};

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
        Err(e) => Err(AppError(e).into()),
    }
}
