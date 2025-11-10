use aes_gcm::{
    Aes256Gcm, KeyInit,
    aead::{Aead, AeadCore, OsRng},
};
use anyhow::Ok;
use async_session::chrono::{Duration, Utc};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    reset_password,
    change_email,
    register,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthInfo {
    user_id: Option<i64>,
    email: Option<String>,
    google_oauth: Option<String>,
    github_oauth: Option<String>,
    action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    exp: i64,
    iat: i64,
    nbf: i64,
    payload: String,
}

impl AuthInfo {
    fn encrypt(&self, encrypt_key: &str) -> anyhow::Result<String> {
        let cipher = Aes256Gcm::new(encrypt_key.as_bytes().into());
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let serialized = serde_json::to_vec(self)?;
        let ciphertext = cipher
            .encrypt(&nonce, serialized.as_ref())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        Ok(general_purpose::STANDARD.encode(&[nonce.as_slice(), ciphertext.as_slice()].concat()))
    }

    fn decrypt(payload: &str, encrypt_key: &str) -> anyhow::Result<Self> {
        let cipher = Aes256Gcm::new(encrypt_key.as_bytes().into());
        let decoded = general_purpose::STANDARD.decode(payload)?;
        let (nonce, ciphertext) = decoded.split_at(12);
        let plaintext = cipher
            .decrypt(nonce.into(), ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(serde_json::from_slice(&plaintext)?)
    }
}

impl AuthToken {
    fn to_jwt(&self, encode_key: &str) -> anyhow::Result<String> {
        let jwt = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(encode_key.as_ref()),
        )?;

        Ok(jwt)
    }

    pub fn verify(jwt: &str, encode_key: &str) -> anyhow::Result<()> {
        jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(())
    }

    pub fn get_action(jwt: &str, encode_key: &str, encrypt_key: &str) -> anyhow::Result<Action> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        let auth_info = AuthInfo::decrypt(&token.claims.payload, encrypt_key)?;

        Ok(auth_info.action)
    }

    pub fn get_email_and_display_id(
        jwt: &str,
        encode_key: &str,
        encrypt_key: &str,
    ) -> anyhow::Result<(Option<String>, Option<i64>)> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        let auth_info = AuthInfo::decrypt(&token.claims.payload, encrypt_key)?;

        Ok((auth_info.email, auth_info.user_id))
    }

    pub fn get_email(
        jwt: &str,
        encode_key: &str,
        encrypt_key: &str,
    ) -> anyhow::Result<Option<String>> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        let auth_info = AuthInfo::decrypt(&token.claims.payload, encrypt_key)?;

        Ok(auth_info.email)
    }

    pub fn get_google_oauth(
        jwt: &str,
        encode_key: &str,
        encrypt_key: &str,
    ) -> anyhow::Result<Option<String>> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        let auth_info = AuthInfo::decrypt(&token.claims.payload, encrypt_key)?;

        Ok(auth_info.google_oauth)
    }

    pub fn get_github_oauth(
        jwt: &str,
        encode_key: &str,
        encrypt_key: &str,
    ) -> anyhow::Result<Option<String>> {
        let token = jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(encode_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        let auth_info = AuthInfo::decrypt(&token.claims.payload, encrypt_key)?;

        Ok(auth_info.github_oauth)
    }

    pub fn encode_email_update_jwt(
        user_id: i64,
        email: &str,
        encode_key: &str,
        encrypt_key: &str,
    ) -> anyhow::Result<String> {
        let exp = (Utc::now() + Duration::minutes(60)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        let auth_info = AuthInfo {
            user_id: Some(user_id),
            email: Some(email.to_string()),
            google_oauth: None,
            github_oauth: None,
            action: Action::change_email,
        };
        let payload = auth_info.encrypt(encrypt_key)?;

        let claims = AuthToken {
            exp,
            iat,
            nbf,
            payload,
        };

        claims.to_jwt(encode_key)
    }

    pub fn encode_signup_jwt(
        email: Option<&str>,
        google_oauth: Option<&str>,
        github_oauth: Option<&str>,
        encode_key: &str,
        encrypt_key: &str,
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

        let auth_info = AuthInfo {
            user_id: None,
            email: email.map(|s| s.to_string()),
            google_oauth: google_oauth.map(|s| s.to_string()),
            github_oauth: github_oauth.map(|s| s.to_string()),
            action: Action::register,
        };

        let payload = auth_info.encrypt(encrypt_key)?;

        let claims = AuthToken {
            exp,
            iat,
            nbf,
            payload,
        };

        claims.to_jwt(encode_key)
    }

    pub fn encode_email_reset_password_jwt(
        email: &str,
        encode_key: &str,
        encrypt_key: &str,
    ) -> anyhow::Result<String> {
        let exp = (Utc::now() + Duration::minutes(60)).timestamp();
        let iat = Utc::now().timestamp();
        let nbf = Utc::now().timestamp();

        let auth_info = AuthInfo {
            user_id: None,
            email: Some(email.to_string()),
            google_oauth: None,
            github_oauth: None,
            action: Action::reset_password,
        };

        let payload = auth_info.encrypt(encrypt_key)?;

        let claims = AuthToken {
            exp,
            iat,
            nbf,
            payload,
        };

        claims.to_jwt(encode_key)
    }
}
