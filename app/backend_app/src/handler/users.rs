use crate::di::DiContainer;
use crate::model::users::{UpdateEmail, UpdateMe, UpdatePassword, UserResponse};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::Cookie};
use reqwest::StatusCode;
use usecase::{
    model::user::{UpdatePasswordData, UpdateUserData},
    service::user::UserError,
};

pub async fn get_me(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container.user_service().get_me(session_id).await {
        Ok(user) => {
            let resp = UserResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            UserError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            UserError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            UserError::ValidateError => Err(StatusCode::BAD_REQUEST),
            UserError::NotFound => Err(StatusCode::NOT_FOUND),
        },
    }
}

pub async fn put_me_email(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(body): Json<UpdateEmail>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container
        .user_service()
        .update_email(session_id, body.email)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            UserError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            UserError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            UserError::ValidateError => Err(StatusCode::BAD_REQUEST),
            UserError::NotFound => Err(StatusCode::NOT_FOUND),
        },
    }
}

pub async fn put_me_password(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(body): Json<UpdatePassword>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container
        .user_service()
        .update_password(
            session_id,
            UpdatePasswordData {
                old_password: body.old_password,
                new_password: body.new_password,
            },
        )
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            UserError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            UserError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            UserError::ValidateError => Err(StatusCode::BAD_REQUEST),
            UserError::NotFound => Err(StatusCode::NOT_FOUND),
        },
    }
}

pub async fn put_me(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(body): Json<UpdateMe>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container
        .user_service()
        .update_me(
            session_id,
            UpdateUserData {
                user_name: body.user_name,
                icon: body.icon,
                x_link: body.x_link,
                github_link: body.github_link,
                self_introduction: body.self_introduction,
            },
        )
        .await
    {
        Ok(user) => {
            let resp = UserResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            UserError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            UserError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            UserError::ValidateError => Err(StatusCode::BAD_REQUEST),
            UserError::NotFound => Err(StatusCode::NOT_FOUND),
        },
    }
}

pub async fn get_user(
    State(di_container): State<DiContainer>,
    Path(display_id): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id");

    match di_container
        .user_service()
        .get_user(display_id, session_id)
        .await
    {
        Ok(user) => {
            let resp = UserResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => match e {
            UserError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            UserError::Unauthorized => Err(StatusCode::UNAUTHORIZED),
            UserError::ValidateError => Err(StatusCode::BAD_REQUEST),
            UserError::NotFound => Err(StatusCode::NOT_FOUND),
        },
    }
}
