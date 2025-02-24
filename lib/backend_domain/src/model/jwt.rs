use anyhow::Ok;
use async_session::chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    reset_password,
    change_email,
    register_email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailToken {
    exp: i64,
    iat: i64,
    nbf: i64,
    user_id: Option<i64>,
    email: String,
    action: Action,
}

impl EmailToken {
    fn to_jwt(&self, encode_key: String) -> anyhow::Result<String> {
        let jwt = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(encode_key.as_ref()),
        )?;

        Ok(jwt)
    }

    pub fn verify(jwt: &str, encode_key: String) -> anyhow::Result<()> {
        jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(())
    }

    pub fn get_email(jwt: &str, encode_key: String) -> anyhow::Result<String> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token.claims.email)
    }

    pub fn encode_email_update_jwt(
        user_id: i64,
        email: &str,
        encode_key: String,
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

        claims.to_jwt(encode_key)
    }

    pub fn encode_email_signup_jwt(email: &str, encode_key: String) -> anyhow::Result<String> {
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

        claims.to_jwt(encode_key)
    }

    pub fn encode_email_reset_password_jwt(
        email: &str,
        encode_key: String,
    ) -> anyhow::Result<String> {
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

        claims.to_jwt(encode_key)
    }
}
