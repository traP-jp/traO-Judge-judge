use std::sync::LazyLock;

use crate::di::DiContainer;
use crate::model::auth::{LogIn, ResetPassword, ResetPasswordRequest, SignUp, SignUpRequest};
use axum::{Json, extract::State, http::HeaderMap, response::IntoResponse};
use axum_extra::{TypedHeader, headers::Cookie};
use reqwest::{StatusCode, header::SET_COOKIE};
use usecase::{
    model::auth::{LoginData, ResetPasswordData, SignUpData},
    service::auth::AuthError,
};


static COOKIE_DOMAIN: LazyLock<String> = LazyLock::new(|| {
    std::env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN must be set")
});

pub async fn signup_request(
    State(di_container): State<DiContainer>,
    Json(body): Json<SignUpRequest>,
) -> impl IntoResponse {
    match di_container.auth_service().signup_request(body.email).await {
        Ok(_) => StatusCode::CREATED,
        Err(e) => match e {
            AuthError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::ValidateError => StatusCode::BAD_REQUEST,
        },
    }
}

pub async fn signup(
    State(di_container): State<DiContainer>,
    Json(body): Json<SignUp>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .auth_service()
        .signup(SignUpData {
            user_name: body.user_name,
            password: body.password,
            token: body.token,
        })
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => match e {
            AuthError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            AuthError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            AuthError::ValidateError => Err(StatusCode::BAD_REQUEST),
        },
    }
}

pub async fn login(
    State(di_container): State<DiContainer>,
    Json(body): Json<LogIn>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .auth_service()
        .login(LoginData {
            email: body.email,
            password: body.password,
        })
        .await
    {
        Ok(session_id) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                SET_COOKIE,
                format!("session_id={session_id}; Domain={}; HttpOnly; SameSite=Lax", *COOKIE_DOMAIN)
                    .parse()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            );

            Ok((StatusCode::NO_CONTENT, headers))
        }
        Err(e) => match e {
            AuthError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            AuthError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            AuthError::ValidateError => Err(StatusCode::BAD_REQUEST),
        },
    }
}

pub async fn logout(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container.auth_service().logout(session_id).await {
        Ok(_) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                SET_COOKIE,
                format!("session_id=; Domain={}; HttpOnly; SameSite=Lax; Max-Age=-1", *COOKIE_DOMAIN)
                    .parse()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            );

            Ok((StatusCode::NO_CONTENT, headers))
        }
        Err(e) => match e {
            AuthError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            AuthError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            AuthError::ValidateError => Err(StatusCode::BAD_REQUEST),
        },
    }
}

pub async fn reset_password_request(
    State(di_container): State<DiContainer>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .auth_service()
        .reset_password_request(body.email)
        .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => match e {
            AuthError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            AuthError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            AuthError::ValidateError => Err(StatusCode::BAD_REQUEST),
        },
    }
}

pub async fn reset_password(
    State(di_container): State<DiContainer>,
    Json(body): Json<ResetPassword>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .auth_service()
        .reset_password(ResetPasswordData {
            password: body.password,
            token: body.token,
        })
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            AuthError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            AuthError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            AuthError::ValidateError => Err(StatusCode::BAD_REQUEST),
        },
    }
}
