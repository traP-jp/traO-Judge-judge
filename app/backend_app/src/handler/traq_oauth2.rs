use crate::di::DiContainer;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use axum_extra::TypedHeader;
use axum_extra::headers::Cookie;
use usecase::service::traq_oauth2::TraqOAuth2Error;

pub async fn post_traq_oauth2_authorize(
    State(di_container): State<DiContainer>,
    Path(oauth_action): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let session_id = cookie.get("session_id");
    let forwarded_user = headers
        .get("X-Forwarded-User")
        .and_then(|v| v.to_str().ok());

    match di_container
        .traq_oauth2_service()
        .post_traq_oauth2_authorize(session_id, &oauth_action, forwarded_user)
        .await
    {
        Ok(response) => {
            if let Some(session_id) = response.session_id {
                let mut headers = HeaderMap::new();
                headers.insert(
                    SET_COOKIE,
                    format!("session_id={session_id}; HttpOnly; Path=/; SameSite=Lax")
                        .parse()
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                );
                Ok((StatusCode::NO_CONTENT, headers).into_response())
            } else {
                Ok(StatusCode::NO_CONTENT.into_response())
            }
        }
        Err(e) => match e {
            TraqOAuth2Error::BadRequest => Err(StatusCode::BAD_REQUEST),
            TraqOAuth2Error::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            TraqOAuth2Error::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn post_traq_oauth2_revoke(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .traq_oauth2_service()
        .post_traq_oauth2_revoke(session_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            TraqOAuth2Error::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            _ => Err(StatusCode::BAD_REQUEST),
        },
    }
}
