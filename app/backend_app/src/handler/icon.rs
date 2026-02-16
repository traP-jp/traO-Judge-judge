use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::{di::DiContainer, model::error::AppError};

pub async fn get_icon(
    State(di_container): State<DiContainer>,
    Path(icon_uuid_str): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let icon_uuid = match uuid::Uuid::parse_str(&icon_uuid_str) {
        Ok(u) => u,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let icon_id = icon_uuid.into();
    match di_container.icon_service().get_icon(icon_id).await {
        Ok(icon) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", icon.content_type.parse().unwrap());
            Ok((headers, icon.icon))
        }
        Err(e) => Err(AppError(e).into()),
    }
}
