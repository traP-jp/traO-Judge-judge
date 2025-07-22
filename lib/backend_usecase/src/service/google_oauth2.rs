use crate::model::google_oauth2::GoogleOAuth2AuthorizeDto;
use domain::model::jwt::EmailToken;
use domain::repository::{auth::AuthRepository, session::SessionRepository, user::UserRepository};

#[derive(Clone)]
pub struct GoogleOAuth2Service<AR: AuthRepository, SR: SessionRepository, UR: UserRepository> {
    auth_repository: AR,
    session_repository: SR,
    user_repository: UR,
}

impl<AR: AuthRepository, SR: SessionRepository, UR: UserRepository>
    GoogleOAuth2Service<AR, SR, UR>
{
    pub fn new(auth_repository: AR, session_repository: SR, user_repository: UR) -> Self {
        Self {
            auth_repository,
            session_repository,
            user_repository,
        }
    }
}

#[derive(Debug)]
pub enum GoogleOAuth2Error {
    BadRequest,
    Unauthorized,
    InternalServerError,
}

impl<AR: AuthRepository, SR: SessionRepository, UR: UserRepository>
    GoogleOAuth2Service<AR, SR, UR>
{
    pub async fn get_google_oauth2_params(
        &self,
        oauth_action: &str,
    ) -> anyhow::Result<String, GoogleOAuth2Error> {
        let url = self
            .auth_repository
            .get_google_oauth2_url(oauth_action)
            .await
            .map_err(|_| GoogleOAuth2Error::InternalServerError)?;
        Ok(url)
    }

    pub async fn post_google_oauth2_authorize(
        &self,
        session_id: Option<&str>,
        oauth_action: &str,
        code: &str,
    ) -> anyhow::Result<GoogleOAuth2AuthorizeDto, GoogleOAuth2Error> {
        let google_oauth = self
            .auth_repository
            .get_google_oauth_by_authorize_code(code, oauth_action)
            .await
            .map_err(|_| GoogleOAuth2Error::BadRequest)?;
        match oauth_action {
            "login" | "signup" => {
                let user_id = self
                    .auth_repository
                    .get_user_id_by_google_oauth(&google_oauth)
                    .await
                    .map_err(|_| GoogleOAuth2Error::InternalServerError)?;
                match user_id {
                    Some(user_id) => {
                        let user = self
                            .user_repository
                            .get_user_by_user_id(user_id)
                            .await
                            .map_err(|_| GoogleOAuth2Error::InternalServerError)?
                            .ok_or(GoogleOAuth2Error::InternalServerError)?;
                        let login_session_id =
                            self.session_repository
                                .create_session(user)
                                .await
                                .map_err(|_| GoogleOAuth2Error::InternalServerError)?;
                        Ok(GoogleOAuth2AuthorizeDto {
                            session_id: Some(login_session_id),
                            token: None,
                        })
                    }
                    None => {
                        let encode_key = std::env::var("JWT_SECRET_KEY").unwrap();
                        let sign_up_token = EmailToken::encode_signup_jwt(
                            None,
                            Some(&google_oauth),
                            None,
                            encode_key,
                        )
                        .map_err(|_| GoogleOAuth2Error::InternalServerError)?;
                        Ok(GoogleOAuth2AuthorizeDto {
                            session_id: None,
                            token: Some(sign_up_token),
                        })
                    }
                }
            }
            "bind" => {
                let session_id = session_id.ok_or(GoogleOAuth2Error::Unauthorized)?;
                let user_id = self
                    .session_repository
                    .get_user_id_by_session_id(session_id)
                    .await
                    .map_err(|_| GoogleOAuth2Error::InternalServerError)?
                    .ok_or(GoogleOAuth2Error::Unauthorized)?;
                self.auth_repository
                    .save_user_google_oauth(user_id, &google_oauth)
                    .await
                    .map_err(|_| GoogleOAuth2Error::InternalServerError)?;
                Ok(GoogleOAuth2AuthorizeDto {
                    session_id: None,
                    token: None,
                })
            }
            _ => Err(GoogleOAuth2Error::InternalServerError),
        }
    }

    pub async fn post_google_oauth2_revoke(
        &self,
        session_id: Option<&str>,
    ) -> anyhow::Result<(), GoogleOAuth2Error> {
        let user_id = if let Some(session_id) = session_id {
            self.session_repository
                .get_user_id_by_session_id(session_id)
                .await
                .map_err(|_| GoogleOAuth2Error::InternalServerError)?
                .ok_or(GoogleOAuth2Error::Unauthorized)?
        } else {
            return Err(GoogleOAuth2Error::Unauthorized);
        };
        if self
            .auth_repository
            .verify_user_google_oauth(user_id)
            .await
            .map_err(|_| GoogleOAuth2Error::BadRequest)?
        {
            return Err(GoogleOAuth2Error::BadRequest);
        }
        if self
            .auth_repository
            .count_authentication_methods(user_id)
            .await
            .map_err(|_| GoogleOAuth2Error::InternalServerError)?
            <= 1
        {
            return Err(GoogleOAuth2Error::BadRequest);
        }
        self.auth_repository
            .delete_user_google_oauth(user_id)
            .await
            .map_err(|_| GoogleOAuth2Error::InternalServerError)?;
        Ok(())
    }
}
