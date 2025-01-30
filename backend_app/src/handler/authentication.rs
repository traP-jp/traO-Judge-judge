use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use lettre::Address;
use reqwest::{header::SET_COOKIE, StatusCode};
use serde::Deserialize;

use crate::{
    utils::validator::{RuleType, Validator},
    Repository,
};

#[derive(Deserialize)]
pub struct SignUpRequest {
    email: String,
}

pub async fn sign_up_request(
    State(state): State<Repository>,
    Json(body): Json<SignUpRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_address = body
        .email
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 既に登録されているメールアドレスのとき、正常時と同じステータスコードを返すが実際にメールを送信しない
    if let Ok(true) = state.is_exist_email(&body.email).await {
        return Ok(StatusCode::CREATED);
    }

    let jwt = state
        .encode_email_signup_jwt(&body.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let message = format!(
        "これはテストメールです。
以下のリンクをクリックしてください。
https://link/{jwt}"
    );

    crate::utils::mail::send_email(user_address, "traOJudgeメール認証", &message)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUp {
    pub user_name: String,
    pub password: String,
    pub token: String,
}

impl Validator for SignUp {
    fn validate(&self) -> anyhow::Result<()> {
        let rules = vec![
            (&self.user_name, RuleType::UserName),
            (&self.password, RuleType::Password),
        ];
        for (field, rule) in rules {
            rule.validate(field)?;
        }
        Ok(())
    }
}

pub async fn sign_up(
    State(state): State<Repository>,
    Json(body): Json<SignUp>,
) -> Result<StatusCode, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
    let email = state
        .get_email_by_email_jwt(&body.token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 既に登録されているメールアドレスのとき、正常時と同じステータスコードを返す
    if let Ok(true) = state.is_exist_email(&email).await {
        return Ok(StatusCode::CREATED);
    }

    let id = state
        .create_user_by_email(&body.user_name, &email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    state
        .save_user_password(id, &body.password)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct LogIn {
    email: String,
    password: String,
}

impl Validator for LogIn {
    fn validate(&self) -> anyhow::Result<()> {
        RuleType::Password.validate(&self.password)?;
        Ok(())
    }
}

pub async fn login(
    State(state): State<Repository>,
    Json(body): Json<LogIn>,
) -> Result<impl IntoResponse, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
    let user = state
        .get_user_by_email(&body.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let verification = state
        .verify_user_password(user.id, &body.password)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !verification {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session_id = state
        .create_session(user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        format!("session_id={}; HttpOnly; SameSite=Lax", session_id)
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok((StatusCode::NO_CONTENT, headers))
}

pub async fn logout(
    State(state): State<Repository>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookie.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;

    state
        .delete_session(session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        "session_id=; HttpOnly; SameSite=Lax; Max-Age=-1"
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok((StatusCode::NO_CONTENT, headers))
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    email: String,
}

pub async fn reset_password_request(
    State(state): State<Repository>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_address = body
        .email
        .parse::<Address>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 登録されていないメールアドレスのとき、正常時と同じステータスコードを返すが実際にメールを送信しない
    if let Ok(false) = state.is_exist_email(&body.email).await {
        return Ok(StatusCode::CREATED);
    }

    let jwt = state
        .encode_email_reset_password_jwt(&body.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let message = format!(
        "これはテストメールです。
以下のリンクをクリックしてください。
https://link/{jwt}"
    );

    crate::utils::mail::send_email(user_address, "traOJudgeパスワードリセット", &message)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct ResetPassword {
    password: String,
    token: String,
}

impl Validator for ResetPassword {
    fn validate(&self) -> anyhow::Result<()> {
        RuleType::Password.validate(&self.password)?;
        Ok(())
    }
}

pub async fn reset_password(
    State(state): State<Repository>,
    Json(body): Json<ResetPassword>,
) -> Result<StatusCode, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    let email = state
        .get_email_by_email_jwt(&body.token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = state
        .get_user_by_email(&email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    state
        .update_user_password(user.id, &body.password)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
