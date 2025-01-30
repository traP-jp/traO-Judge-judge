use anyhow::Ok;
use async_session::chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use super::Repository;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
enum Action {
    reset_password,
    change_email,
    register_email,
}

#[derive(Serialize, Deserialize)]
struct EmailToken {
    exp: i64,
    iat: i64,
    nbf: i64,
    user_id: Option<i64>,
    email: String,
    action: Action,
}

impl EmailToken {
    fn to_jwt(&self) -> anyhow::Result<String> {
        let encode_key: String = std::env::var("JWT_SECRET")?;

        let jwt = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(encode_key.as_ref()),
        )?;

        Ok(jwt)
    }

    fn verify(jwt: &str) -> anyhow::Result<()> {
        let encode_key: String = std::env::var("JWT_SECRET")?;

        jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(())
    }

    fn get_email(jwt: &str) -> anyhow::Result<String> {
        let encode_key: String = std::env::var("JWT_SECRET")?;

        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token.claims.email)
    }
}

impl Repository {
    pub async fn encode_email_update_jwt(
        &self,
        user_id: i64,
        email: &str,
    ) -> anyhow::Result<String> {
        let exp = (Utc::now() + Duration::minutes(60)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        let claims = EmailToken {
            exp,
            iat,
            nbf,
            user_id: Some(user_id),
            email: email.to_string(),
            action: Action::change_email,
        };

        claims.to_jwt()
    }

    pub async fn encode_email_signup_jwt(&self, email: &str) -> anyhow::Result<String> {
        let exp = (Utc::now() + Duration::minutes(60)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        let claims = EmailToken {
            exp,
            iat,
            nbf,
            user_id: None,
            email: email.to_string(),
            action: Action::register_email,
        };

        claims.to_jwt()
    }

    pub async fn encode_email_reset_password_jwt(&self, email: &str) -> anyhow::Result<String> {
        let exp = (Utc::now() + Duration::minutes(60)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        let claims = EmailToken {
            exp,
            iat,
            nbf,
            user_id: None,
            email: email.to_string(),
            action: Action::reset_password,
        };

        claims.to_jwt()
    }

    pub async fn verify_email_jwt(&self, jwt: &str) -> anyhow::Result<()> {
        EmailToken::verify(jwt)
    }
    pub async fn get_email_by_email_jwt(&self, jwt: &str) -> anyhow::Result<String> {
        EmailToken::get_email(jwt)
    }
}
