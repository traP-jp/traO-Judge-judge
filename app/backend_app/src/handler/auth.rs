use crate::model::auth::{LogIn, ResetPassword, ResetPasswordRequest, SignUp, SignUpRequest};
use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use di::DiContainer;
use reqwest::{header::SET_COOKIE, StatusCode};
use usecase::{
    model::auth::{LoginData, ResetPasswordData, SignUpData},
    service::auth::AuthError,
};

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
                format!("session_id={}; HttpOnly; SameSite=Lax", session_id)
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
                "session_id=; HttpOnly; SameSite=Lax; Max-Age=-1"
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
