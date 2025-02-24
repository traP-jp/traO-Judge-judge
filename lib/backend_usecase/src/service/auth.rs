use lettre::Address;

use crate::model::auth::ResetPasswordData;
use crate::model::auth::{LoginData, SignUpData};
use domain::{
    external::mail::MailClient,
    model::jwt::EmailToken,
    repository::session::SessionRepository,
    repository::{auth::AuthRepository, user::UserRepository},
};

#[derive(Clone)]
pub struct AuthenticationService<
    AR: AuthRepository,
    UR: UserRepository,
    SR: SessionRepository,
    C: MailClient,
> {
    auth_repository: AR,
    user_repository: UR,
    session_repository: SR,
    mail_client: C,
}

impl<AR: AuthRepository, UR: UserRepository, SR: SessionRepository, C: MailClient>
    AuthenticationService<AR, UR, SR, C>
{
    pub fn new(
        auth_repository: AR,
        user_repository: UR,
        session_repository: SR,
        mail_client: C,
    ) -> Self {
        Self {
            auth_repository,
            user_repository,
            session_repository,
            mail_client,
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    ValidateError,
    Unauthorized,
    InternalServerError,
}

impl<AR: AuthRepository, UR: UserRepository, SR: SessionRepository, C: MailClient>
    AuthenticationService<AR, UR, SR, C>
{
    pub async fn signup_request(&self, email: String) -> anyhow::Result<(), AuthError> {
        let user_address = email
            .parse::<Address>()
            .map_err(|_| AuthError::ValidateError)?;

        if let Ok(true) = self.user_repository.is_exist_email(&email).await {
            return Ok(());
        }

        let encode_key =
            std::env::var("JWT_SECRET_KEY").map_err(|_| AuthError::InternalServerError)?;

        let jwt = EmailToken::encode_email_signup_jwt(&email, encode_key)
            .map_err(|_| AuthError::InternalServerError)?;

        // todo
        let subject = "Verification mail";
        let message = format!(
            "Please click the link below to verify your email address.\n\n\
            http://localhost:3000/verify?jwt={jwt}"
        );

        self.mail_client
            .send_mail(user_address, subject, &message)
            .await
            .map_err(|_| AuthError::InternalServerError)?;

        Ok(())
    }

    pub async fn signup(&self, data: SignUpData) -> anyhow::Result<(), AuthError> {
        data.validate().map_err(|_| AuthError::ValidateError)?;

        let encode_key =
            std::env::var("JWT_SECRET_KEY").map_err(|_| AuthError::InternalServerError)?;

        let email =
            EmailToken::get_email(&data.token, encode_key).map_err(|_| AuthError::Unauthorized)?;

        if let Ok(true) = self.user_repository.is_exist_email(&email).await {
            return Ok(());
        }

        let user_id = self
            .user_repository
            .create_user_by_email(&data.user_name, &email)
            .await
            .map_err(|_| AuthError::InternalServerError)?;

        self.auth_repository
            .save_user_password(user_id, &data.password)
            .await
            .map_err(|_| AuthError::InternalServerError)?;

        Ok(())
    }

    pub async fn login(&self, data: LoginData) -> anyhow::Result<String, AuthError> {
        data.validate().map_err(|_| AuthError::ValidateError)?;

        let user = self
            .user_repository
            .get_user_by_email(&data.email)
            .await
            .map_err(|_| AuthError::InternalServerError)?
            .ok_or(AuthError::Unauthorized)?;

        if !self
            .auth_repository
            .verify_user_password(user.id, &data.password)
            .await
            .map_err(|_| AuthError::InternalServerError)?
        {
            return Err(AuthError::Unauthorized);
        }

        let session_id = self
            .session_repository
            .create_session(user)
            .await
            .map_err(|_| AuthError::InternalServerError)?;

        Ok(session_id)
    }

    pub async fn logout(&self, session_id: &str) -> anyhow::Result<(), AuthError> {
        self.session_repository
            .delete_session(session_id)
            .await
            .map_err(|_| AuthError::InternalServerError)?
            .ok_or(AuthError::Unauthorized)?;
        Ok(())
    }

    pub async fn reset_password_request(&self, email: String) -> anyhow::Result<(), AuthError> {
        let user_address = email
            .parse::<Address>()
            .map_err(|_| AuthError::ValidateError)?;

        if let Ok(false) = self.user_repository.is_exist_email(&email).await {
            return Ok(());
        }

        let encode_key =
            std::env::var("JWT_SECRET_KEY").map_err(|_| AuthError::InternalServerError)?;

        let jwt = EmailToken::encode_email_reset_password_jwt(&email, encode_key)
            .map_err(|_| AuthError::InternalServerError)?;

        // todo
        let subject = "Reset password mail";
        let message = format!(
            "Please click the link below to reset your password.\n\n\
            http://localhost:3000/reset?jwt={jwt}"
        );

        self.mail_client
            .send_mail(user_address, subject, &message)
            .await
            .map_err(|_| AuthError::InternalServerError)?;

        Ok(())
    }

    pub async fn reset_password(&self, data: ResetPasswordData) -> anyhow::Result<(), AuthError> {
        data.validate().map_err(|_| AuthError::ValidateError)?;

        let encode_key =
            std::env::var("JWT_SECRET_KEY").map_err(|_| AuthError::InternalServerError)?;

        let email =
            EmailToken::get_email(&data.token, encode_key).map_err(|_| AuthError::Unauthorized)?;

        let user = self
            .user_repository
            .get_user_by_email(&email)
            .await
            .map_err(|_| AuthError::InternalServerError)?
            .ok_or(AuthError::Unauthorized)?;

        self.auth_repository
            .update_user_password(user.id, &data.password)
            .await
            .map_err(|_| AuthError::InternalServerError)?;

        Ok(())
    }
}
