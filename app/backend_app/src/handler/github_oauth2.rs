use crate::di::DiContainer;
use crate::model::github_oauth2::{
    GitHubOAuth2AuthorizeRequest, GitHubOAuth2AuthorizeResponse, GitHubOAuth2ParamsResponse,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use axum_extra::TypedHeader;
use axum_extra::headers::Cookie;
use usecase::service::github_oauth2::GitHubOAuth2Error;

pub async fn get_github_oauth2_params(
    State(di_container): State<DiContainer>,
    Path(oauth_action): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .github_oauth2_service()
        .get_github_oauth2_params(&oauth_action)
        .await
    {
        Ok(params) => {
            let resp = GitHubOAuth2ParamsResponse::from(params);
            Ok(Json(resp))
        }
        Err(e) => match e {
            GitHubOAuth2Error::BadRequest => Err(StatusCode::BAD_REQUEST),
            GitHubOAuth2Error::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            GitHubOAuth2Error::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn post_github_oauth2_authorize(
    State(di_container): State<DiContainer>,
    Path(oauth_action): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(query): Json<GitHubOAuth2AuthorizeRequest>,
) -> Result<Response, StatusCode> {
    let session_id = cookie.get("session_id");
    match di_container
        .github_oauth2_service()
        .post_github_oauth2_authorize(session_id, &oauth_action, &query.code)
        .await
    {
        Ok(response) => {
            if let Some(login_session_id) = response.session_id {
                let mut headers = HeaderMap::new();
                headers.insert(
                    SET_COOKIE,
                    format!("session_id={}; HttpOnly; SameSite=Lax", login_session_id)
                        .parse()
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                );
                Ok((StatusCode::NO_CONTENT, headers).into_response())
            } else if response.token.is_some() {
                let resp = GitHubOAuth2AuthorizeResponse::from(response);
                Ok((StatusCode::OK, Json(resp)).into_response())
            } else {
                Ok(StatusCode::NO_CONTENT.into_response())
            }
        }
        Err(e) => match e {
            GitHubOAuth2Error::BadRequest => Err(StatusCode::BAD_REQUEST),
            GitHubOAuth2Error::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            GitHubOAuth2Error::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn post_github_oauth2_revoke(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .github_oauth2_service()
        .post_github_oauth2_revoke(session_id)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            GitHubOAuth2Error::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            _ => Err(StatusCode::BAD_REQUEST),
        },
    }
}
