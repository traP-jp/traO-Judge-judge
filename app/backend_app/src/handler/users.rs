use crate::di::DiContainer;
use crate::model::error::AppError;
use crate::model::users::{UpdateEmail, UpdateMe, UpdatePassword, UserMeResponse, UserResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::Cookie};
use usecase::model::user::{UpdatePasswordData, UpdateUserData};

pub async fn get_me(
    State(di_container): State<DiContainer>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    match di_container.user_service().get_me(session_id).await {
        Ok(user) => {
            let resp = UserMeResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
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
        Err(e) => Err(AppError(e).into()),
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
        Err(e) => Err(AppError(e).into()),
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
                github_id: if body.github_id.is_empty() {
                    None
                } else {
                    Some(body.github_id)
                },
                x_id: if body.x_id.is_empty() {
                    None
                } else {
                    Some(body.x_id)
                },
                self_introduction: body.self_introduction,
            },
        )
        .await
    {
        Ok(user) => {
            let resp = UserMeResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
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
        Err(e) => Err(AppError(e).into()),
    }
}
