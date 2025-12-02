use crate::model::{error::UsecaseError, traq_oauth2::TraqOAuth2AuthorizeDto};
use domain::{
    model::user::UserRole,
    repository::{auth::AuthRepository, session::SessionRepository, user::UserRepository},
};

#[derive(Clone)]
pub struct TraqOAuth2Service<AR: AuthRepository, SR: SessionRepository, UR: UserRepository> {
    auth_repository: AR,
    session_repository: SR,
    user_repository: UR,
}

impl<AR: AuthRepository, SR: SessionRepository, UR: UserRepository> TraqOAuth2Service<AR, SR, UR> {
    pub fn new(auth_repository: AR, session_repository: SR, user_repository: UR) -> Self {
        Self {
            auth_repository,
            session_repository,
            user_repository,
        }
    }
}

impl<AR: AuthRepository, SR: SessionRepository, UR: UserRepository> TraqOAuth2Service<AR, SR, UR> {
    pub async fn post_traq_oauth2_authorize(
        &self,
        session_id: Option<&str>,
        oauth_action: &str,
        x_forwarded_user: Option<&str>,
    ) -> anyhow::Result<TraqOAuth2AuthorizeDto, UsecaseError> {
        let traq_oauth = x_forwarded_user.ok_or(UsecaseError::BadRequest)?;
        if traq_oauth.is_empty() {
            return Err(UsecaseError::BadRequest);
        }

        match oauth_action {
            "login" => {
                let user_id = self
                    .auth_repository
                    .get_user_id_by_traq_oauth(traq_oauth)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?
                    .ok_or(UsecaseError::Unauthorized)?;

                let user = self
                    .user_repository
                    .get_user_by_user_id(user_id)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?
                    .ok_or_else(|| {
                        UsecaseError::internal_server_error_msg(
                            "user not found by user_id during traQ OAuth2 login",
                        )
                    })?;

                let login_session_id = self
                    .session_repository
                    .create_session(user)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;

                Ok(TraqOAuth2AuthorizeDto {
                    session_id: Some(login_session_id),
                })
            }

            "signup" => {
                let user_id = self
                    .auth_repository
                    .get_user_id_by_traq_oauth(traq_oauth)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;
                if user_id.is_some() {
                    return Err(UsecaseError::ConflictError);
                }

                let new_user_id = self
                    .user_repository
                    .create_user(traq_oauth)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;

                self.auth_repository
                    .save_user_traq_oauth(new_user_id, traq_oauth)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;

                self.user_repository
                    .change_user_role(new_user_id, UserRole::TrapUser)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;

                let user = self
                    .user_repository
                    .get_user_by_user_id(new_user_id)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?
                    .ok_or_else(|| UsecaseError::internal_server_error_msg("user not found by user_id after creating user during traQ OAuth2 signup"))?;

                let login_session_id = self
                    .session_repository
                    .create_session(user)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;

                Ok(TraqOAuth2AuthorizeDto {
                    session_id: Some(login_session_id),
                })
            }

            "bind" => {

                // 現状mergeはできない
                let user_id = self
                    .auth_repository
                    .get_user_id_by_traq_oauth(traq_oauth)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;
                if user_id.is_some() {
                    return Err(UsecaseError::ConflictError);
                }
                

                let session_id = session_id.ok_or(UsecaseError::Unauthorized)?;
                let user_id = self
                    .session_repository
                    .get_user_id_by_session_id(session_id)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?
                    .ok_or(UsecaseError::Unauthorized)?;

                self.auth_repository
                    .update_user_traq_oauth(user_id, traq_oauth)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;

                self.user_repository
                    .change_user_role(user_id, UserRole::TrapUser)
                    .await
                    .map_err(UsecaseError::internal_server_error_map())?;

                Ok(TraqOAuth2AuthorizeDto { session_id: None })
            }
            _ => Err(UsecaseError::internal_server_error_msg(
                "invalid oauth_action for traQ OAuth2 authorize",
            )),
        }
    }

    pub async fn post_traq_oauth2_revoke(
        &self,
        session_id: Option<&str>,
    ) -> anyhow::Result<(), UsecaseError> {
        let user_id = if let Some(session_id) = session_id {
            self.session_repository
                .get_user_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?
                .ok_or(UsecaseError::BadRequest)?
        } else {
            return Err(UsecaseError::BadRequest);
        };

        if !self
            .auth_repository
            .verify_user_traq_oauth(user_id)
            .await
            .map_err(|_| UsecaseError::BadRequest)?
        {
            return Err(UsecaseError::BadRequest);
        }

        if self
            .auth_repository
            .count_authentication_methods(user_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            <= 1
        {
            return Err(UsecaseError::BadRequest);
        }

        self.user_repository
            .change_user_role(user_id, UserRole::CommonUser)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        self.auth_repository
            .delete_user_traq_oauth(user_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;
        Ok(())
    }
}
