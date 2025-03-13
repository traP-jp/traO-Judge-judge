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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod signup_request_tests {
    use super::*;
    use domain::{
        external::mail::MockMailClient,
        repository::{
            auth::MockAuthRepository, session::MockSessionRepository, user::MockUserRepository,
        },
    };
    use rstest::*;
    #[fixture]
    fn setup_env() -> () {
        std::env::set_var("JWT_SECRET_KEY", "secret_test");
    }

    #[rstest]
    #[case::valid_data("test@example.com", Ok(()))]
    #[case::valid_data("x!&x@example.com", Ok(()))]
    #[case::valid_data("0test++--.__1@example.com", Ok(()))]
    #[case::invalid_email("test+-.._1@example.com", Err(AuthError::ValidateError))]
    #[case::invalid_email("test.example.com", Err(AuthError::ValidateError))]
    async fn signup_request(
        _setup_env: (),
        #[case] email: String,
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mut mail_mock = MockMailClient::new();

        mail_mock.expect_send_mail().returning(|_, _, _| Ok(()));
        user_mock.expect_is_exist_email().returning(|_| Ok(false));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.signup_request(email).await;

        assert_eq!(resp, result);

        Ok(())
    }

    #[rstest]
    #[case::valid_data("test@example.com", Ok(()))]
    #[case::valid_data("x!&x@example.com", Ok(()))]
    #[case::invalid_email("test+-.._1@example.com", Err(AuthError::ValidateError))]
    #[case::invalid_email("test.example.com", Err(AuthError::ValidateError))]
    async fn signup_request_exist_user(
        #[case] email: String,
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        user_mock.expect_is_exist_email().returning(|_| Ok(true));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.signup_request(email).await;

        assert_eq!(resp, result);

        Ok(())
    }
}

#[cfg(test)]
mod signup_tests {
    use super::*;
    use domain::{
        external::mail::MockMailClient,
        model::{jwt::EmailToken, user::UserId},
        repository::{
            auth::MockAuthRepository, session::MockSessionRepository, user::MockUserRepository,
        },
    };
    use rstest::*;
    use uuid::Uuid;
    #[fixture]
    fn setup_env() -> () {
        std::env::set_var("JWT_SECRET_KEY", "secret_test");
    }

    fn create_signup_data(user_name: &str, password: &str, email: &str) -> SignUpData {
        let encode_key = std::env::var("JWT_SECRET_KEY").unwrap();
        SignUpData {
            user_name: user_name.to_string(),
            password: password.to_string(),
            token: EmailToken::encode_email_signup_jwt(email, encode_key).unwrap(),
        }
    }

    // email は signup_request でvalidか判定されたのちjwtになり変更不能なのでテストしない
    #[rstest]
    #[case::valid_data(("test", "Passw0rd", "test@example.com"), Ok(()))]
    #[case::valid_data(("1234567890", "Aa0@$!%*?&", "test@gmail.com"), Ok(()))]
    #[case::invalid_password(("test", "Aa12345", "test@example.com"), Err(AuthError::ValidateError))]
    #[case::invalid_password(("test", "@$!%*?&@$", "test@example.com"), Err(AuthError::ValidateError))]
    #[case::invalid_username(("_Alice", "Aa123456", "test@example.com"), Err(AuthError::ValidateError))]
    #[case::invalid_username(("test/Test", "Aa123456", "test@example.com"), Err(AuthError::ValidateError))]
    async fn signup(
        _setup_env: (),
        #[case] data: (&str, &str, &str),
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let signup_data = create_signup_data(data.0, data.1, data.2);

        let mut auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        user_mock.expect_is_exist_email().returning(|_| Ok(false));
        user_mock
            .expect_create_user_by_email()
            .returning(|_, _| Ok(UserId::new(Uuid::now_v7())));
        auth_mock
            .expect_save_user_password()
            .returning(|_, _| Ok(()));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.signup(signup_data).await;

        assert_eq!(resp, result);

        Ok(())
    }

    #[rstest]
    #[case::valid_data(("test", "Passw0rd", "test@example.com"), Ok(()))]
    #[case::valid_data(("1234567890", "Aa0@$!%*?&", "test@gmail.com"), Ok(()))]
    #[case::invalid_password(("test", "Aa12345", "test@example.com"), Err(AuthError::ValidateError))]
    #[case::invalid_password(("test", "@$!%*?&@$", "test@example.com"), Err(AuthError::ValidateError))]
    #[case::invalid_username(("_Alice", "Aa123456", "test@example.com"), Err(AuthError::ValidateError))]
    #[case::invalid_username(("test/Test", "Aa123456", "test@example.com"), Err(AuthError::ValidateError))]
    async fn signup_exist_user(
        _setup_env: (),
        #[case] data: (&str, &str, &str),
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let signup_data = create_signup_data(data.0, data.1, data.2);

        let auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        user_mock.expect_is_exist_email().returning(|_| Ok(true));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.signup(signup_data).await;

        assert_eq!(resp, result);

        Ok(())
    }
}

#[cfg(test)]
mod login_tests {
    use super::*;
    use domain::{
        external::mail::MockMailClient,
        model::user::{User, UserId, UserRole},
        repository::{
            auth::MockAuthRepository, session::MockSessionRepository, user::MockUserRepository,
        },
    };
    use rstest::*;
    use sqlx::types::chrono;
    use uuid::Uuid;

    fn get_user() -> User {
        User {
            id: UserId(Uuid::now_v7()),
            display_id: 0,
            name: "name".to_string(),
            traq_id: None,
            github_id: None,
            icon_url: None,
            x_link: None,
            github_link: None,
            self_introduction: "".to_string(),
            role: UserRole::CommonUser,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn create_login_data(email: &str, password: &str) -> LoginData {
        LoginData {
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    #[rstest]
    #[case::valid_data(("test@sample.com", "Aa123456"), Ok("session_id".to_string()))]
    #[case::valid_data(("t@t", "Aa123456"), Ok("session_id".to_string()))]
    #[case::invalid_password(("test@sample.com", "Aa12345"), Err(AuthError::ValidateError))]
    #[case::invalid_password(("t@t", "aa123456"), Err(AuthError::ValidateError))]
    async fn login(
        #[case] data: (&str, &str),
        #[case] result: Result<String, AuthError>,
    ) -> anyhow::Result<()> {
        let login_data = create_login_data(data.0, data.1);

        let mut auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let mut session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        user_mock
            .expect_get_user_by_email()
            .returning(|_| Ok(Some(get_user())));
        auth_mock
            .expect_verify_user_password()
            .returning(|_, _| Ok(true));
        session_mock
            .expect_create_session()
            .returning(|_| Ok("session_id".to_string()));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.login(login_data).await;

        assert_eq!(resp, result);

        Ok(())
    }

    #[rstest]
    #[case::valid_data(("test@sample.com", "Aa123456"), Err(AuthError::Unauthorized))]
    #[case::valid_data(("t@t", "Aa123456"), Err(AuthError::Unauthorized))]
    #[case::invalid_password(("test@sample.com", "Aa12345"), Err(AuthError::ValidateError))]
    #[case::invalid_password(("t@t", "aa123456"), Err(AuthError::ValidateError))]
    async fn login_not_exist_user(
        #[case] data: (&str, &str),
        #[case] result: Result<String, AuthError>,
    ) -> anyhow::Result<()> {
        let login_data = create_login_data(data.0, data.1);

        let auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        user_mock.expect_get_user_by_email().returning(|_| Ok(None));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.login(login_data).await;

        assert_eq!(resp, result);

        Ok(())
    }

    #[rstest]
    #[case::valid_data(("test@sample.com", "Aa123456"), Err(AuthError::Unauthorized))]
    #[case::valid_data(("t@t", "Aa123456"), Err(AuthError::Unauthorized))]
    async fn login_wrong_password(
        #[case] data: (&str, &str),
        #[case] result: Result<String, AuthError>,
    ) -> anyhow::Result<()> {
        let login_data = create_login_data(data.0, data.1);

        let mut auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        user_mock
            .expect_get_user_by_email()
            .returning(|_| Ok(Some(get_user())));
        auth_mock
            .expect_verify_user_password()
            .returning(|_, _| Ok(false));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.login(login_data).await;

        assert_eq!(resp, result);

        Ok(())
    }
}

#[cfg(test)]
mod logout_tests {
    use super::*;
    use domain::{
        external::mail::MockMailClient,
        repository::{
            auth::MockAuthRepository, session::MockSessionRepository, user::MockUserRepository,
        },
    };
    use rstest::*;

    #[rstest]
    #[case::valid_data("session_id", Ok(()))]
    async fn logout(
        #[case] session_id: &str,
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let auth_mock = MockAuthRepository::new();
        let user_mock = MockUserRepository::new();
        let mut session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        session_mock
            .expect_delete_session()
            .returning(|_| Ok(Some(())));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.logout(session_id).await;

        assert_eq!(resp, result);

        Ok(())
    }

    #[rstest]
    #[case::valid_data("session_id", Err(AuthError::Unauthorized))]
    async fn logout_not_exist_session(
        #[case] session_id: &str,
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let auth_mock = MockAuthRepository::new();
        let user_mock = MockUserRepository::new();
        let mut session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        session_mock.expect_delete_session().returning(|_| Ok(None));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.logout(session_id).await;

        assert_eq!(resp, result);

        Ok(())
    }
}

#[cfg(test)]
mod reset_password_request_tests {
    use super::*;
    use domain::{
        external::mail::MockMailClient,
        repository::{
            auth::MockAuthRepository, session::MockSessionRepository, user::MockUserRepository,
        },
    };
    use rstest::*;
    #[fixture]
    fn setup_env() -> () {
        std::env::set_var("JWT_SECRET_KEY", "secret_test");
    }

    #[rstest]
    #[case::valid_data("test@example.com", Ok(()))]
    #[case::valid_data("x!&x@example.com", Ok(()))]
    #[case::invalid_email("test+-.._1@example.com", Err(AuthError::ValidateError))]
    #[case::invalid_email("test.example.com", Err(AuthError::ValidateError))]
    async fn reset_password_request(
        _setup_env: (),
        #[case] email: &str,
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mut mail_mock = MockMailClient::new();

        mail_mock.expect_send_mail().returning(|_, _, _| Ok(()));
        user_mock.expect_is_exist_email().returning(|_| Ok(true));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.reset_password_request(email.to_string()).await;

        assert_eq!(resp, result);

        Ok(())
    }

    #[rstest]
    #[case::valid_data("test@example.com", Ok(()))]
    #[case::invalid_email("test+-.._1@example.com", Err(AuthError::ValidateError))]
    async fn reset_password_request_not_exist_user(
        #[case] email: &str,
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mut mail_mock = MockMailClient::new();

        mail_mock.expect_send_mail().returning(|_, _, _| Ok(()));
        user_mock.expect_is_exist_email().returning(|_| Ok(false));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.reset_password_request(email.to_string()).await;

        assert_eq!(resp, result);

        Ok(())
    }
}

#[cfg(test)]
mod reset_password_tests {

    use super::*;
    use domain::{
        external::mail::MockMailClient,
        model::user::{User, UserId, UserRole},
        repository::{
            auth::MockAuthRepository, session::MockSessionRepository, user::MockUserRepository,
        },
    };
    use rstest::*;
    use sqlx::types::chrono;
    use uuid::Uuid;
    #[fixture]
    fn setup_env() -> () {
        std::env::set_var("JWT_SECRET_KEY", "secret_test");
    }

    fn create_reset_password_data(email: &str, password: &str) -> ResetPasswordData {
        let encode_key = std::env::var("JWT_SECRET_KEY").unwrap();
        ResetPasswordData {
            password: password.to_string(),
            token: EmailToken::encode_email_reset_password_jwt(email, encode_key.to_string())
                .unwrap(),
        }
    }

    fn get_user() -> User {
        User {
            id: UserId(Uuid::now_v7()),
            display_id: 0,
            name: "name".to_string(),
            traq_id: None,
            github_id: None,
            icon_url: None,
            x_link: None,
            github_link: None,
            self_introduction: "".to_string(),
            role: UserRole::CommonUser,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[rstest]
    #[case::valid_data(("test@sample.com", "Aa123456"), Ok(()))]
    #[case::valid_data(("t@t", "Aa123456"), Ok(()))]
    #[case::invalid_password(("test@sample.com", "Aa12345"), Err(AuthError::ValidateError))]
    #[case::invalid_password(("t@t", "aa123456"), Err(AuthError::ValidateError))]
    async fn reset_password_request(
        _setup_env: (),
        #[case] data: (&str, &str),
        #[case] result: Result<(), AuthError>,
    ) -> anyhow::Result<()> {
        let reset_data = create_reset_password_data(data.0, data.1);

        let mut auth_mock = MockAuthRepository::new();
        let mut user_mock = MockUserRepository::new();
        let session_mock = MockSessionRepository::new();
        let mail_mock = MockMailClient::new();

        user_mock
            .expect_get_user_by_email()
            .returning(|_| Ok(Some(get_user())));
        auth_mock
            .expect_update_user_password()
            .returning(|_, _| Ok(()));

        let service = AuthenticationService::new(auth_mock, user_mock, session_mock, mail_mock);
        let resp = service.reset_password(reset_data).await;

        assert_eq!(resp, result);

        Ok(())
    }
}
