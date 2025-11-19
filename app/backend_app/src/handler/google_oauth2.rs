use crate::di::DiContainer;
use crate::model::error::AppError;
use crate::model::google_oauth2::{
    GoogleOAuth2AuthorizeRequest, GoogleOAuth2AuthorizeResponse, GoogleOAuth2ParamsResponse,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use axum_extra::TypedHeader;
use axum_extra::headers::Cookie;

pub async fn get_google_oauth2_params(
    State(di_container): State<DiContainer>,
    Path(oauth_action): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .google_oauth2_service()
        .get_google_oauth2_params(&oauth_action)
        .await
    {
        Ok(params) => {
            let resp = GoogleOAuth2ParamsResponse::from(params);
            Ok(Json(resp))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn post_google_oauth2_authorize(
    State(di_container): State<DiContainer>,
    Path(oauth_action): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(query): Json<GoogleOAuth2AuthorizeRequest>,
) -> Result<Response, StatusCode> {
    let session_id = cookie.get("session_id");
    match di_container
        .google_oauth2_service()
        .post_google_oauth2_authorize(session_id, &oauth_action, &query.code)
        .await
    {
        Ok(response) => {
            if let Some(login_session_id) = response.session_id {
                let mut headers = HeaderMap::new();
                headers.insert(
                    SET_COOKIE,
                    format!(
                        "session_id={}; HttpOnly; Path=/; SameSite=Lax",
                        login_session_id
                    )
                    .parse()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                );
                Ok((StatusCode::NO_CONTENT, headers).into_response())
            } else if response.token.is_some() {
                let resp = GoogleOAuth2AuthorizeResponse::from(response);
                Ok((StatusCode::OK, Json(resp)).into_response())
            } else {
                Ok(StatusCode::NO_CONTENT.into_response())
            }
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn post_google_oauth2_revoke(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .google_oauth2_service()
        .post_google_oauth2_revoke(session_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}
