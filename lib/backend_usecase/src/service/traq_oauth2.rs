use crate::model::traq_oauth2::TraqOAuth2AuthorizeDto;
use domain::repository::{auth::AuthRepository, session::SessionRepository, user::UserRepository};

#[derive(Clone)]
pub struct TraqOAuth2Service<AR: AuthRepository, SR: SessionRepository, UR: UserRepository> {
    auth_repository: AR,
    session_repository: SR,
    user_repository: UR,
}

impl<AR: AuthRepository, SR: SessionRepository, UR: UserRepository>
    TraqOAuth2Service<AR, SR, UR>
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
pub enum TraqOAuth2Error {
    BadRequest,
    Unauthorized,
    InternalServerError,
}

impl<AR: AuthRepository, SR: SessionRepository, UR: UserRepository>
    TraqOAuth2Service<AR, SR, UR>
{
    pub async fn post_traq_oauth2_authorize(
        &self,
        session_id: Option<&str>,
        oauth_action: &str,
        x_forwarded_user: Option<&str>,
    ) -> anyhow::Result<TraqOAuth2AuthorizeDto, TraqOAuth2Error> {
        let traq_oauth = x_forwarded_user.ok_or(TraqOAuth2Error::BadRequest)?;

        tracing::debug!("X-Forwarded-User: {:?}", traq_oauth);
        
        match oauth_action {
            "login" => {
                let user_id = self
                    .auth_repository
                    .get_user_id_by_traq_oauth(traq_oauth)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?
                    .ok_or(TraqOAuth2Error::Unauthorized)?;

                let user = self
                    .user_repository
                    .get_user_by_user_id(user_id)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?
                    .ok_or(TraqOAuth2Error::InternalServerError)?;

                let login_session_id =
                    self.session_repository
                        .create_session(user)
                        .await
                        .map_err(|_| TraqOAuth2Error::InternalServerError)?;

                Ok(TraqOAuth2AuthorizeDto {
                    session_id: Some(login_session_id)
                })
            }
            
            "signup" => {
                let user_id = self
                    .auth_repository
                    .get_user_id_by_traq_oauth(traq_oauth)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?;
                if user_id.is_some() {
                    return Err(TraqOAuth2Error::BadRequest);
                }

                let new_user_id = self
                    .user_repository
                    .create_user_without_email(traq_oauth)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?;

                self.auth_repository
                    .save_user_traq_oauth(new_user_id, traq_oauth)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?;

                let user = self
                    .user_repository
                    .get_user_by_user_id(new_user_id)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?
                    .ok_or(TraqOAuth2Error::InternalServerError)?;

                let login_session_id =
                    self.session_repository
                        .create_session(user)
                        .await
                        .map_err(|_| TraqOAuth2Error::InternalServerError)?;

                Ok(TraqOAuth2AuthorizeDto {
                    session_id: Some(login_session_id)
                })
            }

            "bind" => {
                let session_id = session_id.ok_or(TraqOAuth2Error::Unauthorized)?;
                let user_id = self
                    .session_repository
                    .get_user_id_by_session_id(session_id)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?
                    .ok_or(TraqOAuth2Error::Unauthorized)?;
                self.auth_repository
                    .update_user_traq_oauth(user_id, traq_oauth)
                    .await
                    .map_err(|_| TraqOAuth2Error::InternalServerError)?;
                Ok(TraqOAuth2AuthorizeDto {
                    session_id: None
                })
            }
            _ => Err(TraqOAuth2Error::InternalServerError),
        }
    }

    pub async fn post_traq_oauth2_revoke(
        &self,
        session_id: Option<&str>,
    ) -> anyhow::Result<(), TraqOAuth2Error> {
        let user_id = if let Some(session_id) = session_id {
            self.session_repository
                .get_user_id_by_session_id(session_id)
                .await
                .map_err(|_| TraqOAuth2Error::InternalServerError)?
                .ok_or(TraqOAuth2Error::Unauthorized)?
        } else {
            return Err(TraqOAuth2Error::Unauthorized);
        };

        if !self
            .auth_repository
            .verify_user_traq_oauth(user_id)
            .await
            .map_err(|_| TraqOAuth2Error::BadRequest)?
        {
            return Err(TraqOAuth2Error::BadRequest);
        }

        if self
            .auth_repository
            .count_authentication_methods(user_id)
            .await
            .map_err(|_| TraqOAuth2Error::InternalServerError)?
            <= 1
        {
            return Err(TraqOAuth2Error::BadRequest);
        }
        self.auth_repository
            .delete_user_traq_oauth(user_id)
            .await
            .map_err(|_| TraqOAuth2Error::InternalServerError)?;
        Ok(())
    }
}
