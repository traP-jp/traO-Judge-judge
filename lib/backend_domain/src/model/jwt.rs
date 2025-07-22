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
    email: Option<String>,
    google_oauth: Option<String>,
    github_oauth: Option<String>,
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

    pub fn get_email(jwt: &str, encode_key: String) -> anyhow::Result<Option<String>> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token.claims.email)
    }

    pub fn get_google_oauth(jwt: &str, encode_key: String) -> anyhow::Result<Option<String>> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token.claims.google_oauth)
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
            email: Some(email.to_string()),
            google_oauth: None,
            github_oauth: None,
            action: Action::change_email,
        };

        claims.to_jwt(encode_key)
    }

    pub fn encode_signup_jwt(
        email: Option<&str>,
        google_oauth: Option<&str>,
        github_oauth: Option<&str>,
        encode_key: String,
    ) -> anyhow::Result<String> {
        let exp = (Utc::now() + Duration::minutes(60)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        let auth_method_count = [email, google_oauth, github_oauth].iter().flatten().count();

        if auth_method_count != 1 {
            return Err(anyhow::anyhow!(
                "At least one authentication method must be provided"
            ));
        }

        let claims = EmailToken {
            exp,
            iat,
            nbf,
            user_id: None,
            email: email.map(|s| s.to_string()),
            google_oauth: google_oauth.map(|s| s.to_string()),
            github_oauth: github_oauth.map(|s| s.to_string()),
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
            email: Some(email.to_string()),
            google_oauth: None,
            github_oauth: None,
            action: Action::reset_password,
        };

        claims.to_jwt(encode_key)
    }
}
