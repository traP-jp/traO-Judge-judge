use crate::di::DiContainer;
use crate::model::language::LanguageResponse;
use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;

pub async fn get_languages(
    State(di_container): State<DiContainer>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container.language_service().get_language().await {
        Ok(languages) => {
            let resp: Vec<LanguageResponse> = languages.into_iter().map(|l| l.into()).collect();
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
