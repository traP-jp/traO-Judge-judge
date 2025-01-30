use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use lettre::Address;
use reqwest::StatusCode;
use serde::Deserialize;

use super::Repository;
use crate::repository::users::UpdateUser;
use crate::utils::validator::{RuleType, Validator};

pub async fn get_me(
    State(state): State<Repository>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    let display_id = state
        .get_display_id_by_session_id(session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = state
        .get_user_by_display_id(display_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct EmailUpdate {
    email: String,
}

pub async fn put_me_email(
    State(state): State<Repository>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(body): Json<EmailUpdate>,
) -> anyhow::Result<StatusCode, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    let display_id = state
        .get_display_id_by_session_id(session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let email = body
        .email
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 既に登録されているメールアドレスのとき、正常時と同じステータスコードを返すが実際にメールを送信しない
    if let Ok(true) = state.is_exist_email(&body.email).await {
        return Ok(StatusCode::NO_CONTENT);
    }

    let jwt = state
        .encode_email_update_jwt(display_id, &body.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let message = format!(
        "以下のリンクをクリックして、メールアドレスの変更を確認してください。
https://link/{jwt}"
    );

    crate::utils::mail::send_email(email, "「traOJudge」メール変更の確認", &message)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordUpdate {
    old_password: String,
    new_password: String,
}

impl Validator for PasswordUpdate {
    fn validate(&self) -> anyhow::Result<()> {
        RuleType::Password.validate(&self.old_password)?;
        RuleType::Password.validate(&self.new_password)?;
        Ok(())
    }
}

pub async fn put_me_password(
    State(state): State<Repository>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(body): Json<PasswordUpdate>,
) -> anyhow::Result<StatusCode, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    let id = state
        .get_user_id_by_session_id(session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match state.verify_user_password(id, &body.old_password).await {
        Ok(true) => {
            state
                .update_user_password(id, &body.new_password)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(StatusCode::NO_CONTENT)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PutMeRequest {
    pub user_name: Option<String>,
    pub icon: Option<String>,
    pub x_link: Option<String>,
    pub github_link: Option<String>,
    pub self_introduction: Option<String>,
}

impl Validator for PutMeRequest {
    fn validate(&self) -> anyhow::Result<()> {
        let rules = vec![
            (&self.user_name, RuleType::UserName),
            (&self.icon, RuleType::Icon),
            (&self.x_link, RuleType::XLink),
            (&self.github_link, RuleType::GitHubLink),
            (&self.self_introduction, RuleType::SelfIntroduction),
        ];
        for (value, rule) in rules {
            if let Some(value) = value {
                rule.validate(value)?;
            }
        }
        Ok(())
    }
}

// todo とりえずの仮置き
fn encode_icon_to_icon_url(icon: Option<String>) -> Option<String> {
    icon
}

pub async fn put_me(
    State(state): State<Repository>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(body): Json<PutMeRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    let display_id = state
        .get_display_id_by_session_id(session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = state
        .get_user_by_display_id(display_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_body = UpdateUser {
        user_name: body.user_name.unwrap_or(user.name),
        icon_url: body
            .icon
            .map_or(user.icon_url, |icon| encode_icon_to_icon_url(Some(icon))),
        x_link: body.x_link.or(user.x_link),
        github_link: body.github_link.or(user.github_link),
        self_introduction: body.self_introduction.unwrap_or(user.self_introduction),
    };

    state
        .update_user(user.display_id, new_body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = state
        .get_user_by_display_id(display_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(new_user))
}

pub async fn get_user(
    State(state): State<Repository>,
    Path(display_id): Path<String>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let display_id = display_id
        .parse::<i64>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let user = state
        .get_user_by_display_id(display_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}
