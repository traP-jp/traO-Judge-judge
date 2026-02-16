use crate::di::DiContainer;
use crate::extractor::session::ExtractedSessionUser;
use crate::model::error::AppError;
use crate::model::users::{UpdateEmail, UpdateMe, UpdatePassword, UserMeResponse, UserResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use usecase::model::user::{UpdatePasswordData, UpdateUserData};

pub async fn get_me(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container.user_service().get_me(session_user).await {
        Ok(user) => {
            let resp = UserMeResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn put_me_email(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Json(body): Json<UpdateEmail>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .user_service()
        .update_email(session_user, body.email)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError(e).into()),
    }
}

pub async fn put_me_password(
    State(di_container): State<DiContainer>,
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Json(body): Json<UpdatePassword>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .user_service()
        .update_password(
            session_user,
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
    ExtractedSessionUser(session_user): ExtractedSessionUser,
    Json(body): Json<UpdateMe>,
) -> Result<impl IntoResponse, StatusCode> {
    match di_container
        .user_service()
        .update_me(
            session_user,
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
    ExtractedSessionUser(session_user): ExtractedSessionUser,
) -> Result<impl IntoResponse, StatusCode> {
    let display_id = display_id
        .parse::<i64>()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into();
    match di_container
        .user_service()
        .get_user(display_id, session_user)
        .await
    {
        Ok(user) => {
            let resp = UserResponse::from(user);
            Ok((StatusCode::OK, Json(resp)))
        }
        Err(e) => Err(AppError(e).into()),
    }
}
