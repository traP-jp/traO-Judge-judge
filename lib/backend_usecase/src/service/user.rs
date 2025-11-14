use std::sync::Arc;

use base64::{Engine, prelude::BASE64_STANDARD};
use lettre::Address;

use crate::{
    model::{
        error::UsecaseError, problem::NormalProblemsDto, submission::SubmissionsDto, user::{UpdatePasswordData, UpdateUserData, UserDto, UserMeDto}
    },
    service::auth_mail_template::{AuthMailTemplateProvider, DefaultAuthMailTemplateProvider},
};
use domain::{
    external::mail::MailClient,
    model::{
        jwt::AuthToken, problem::ProblemGetQuery, submission::SubmissionGetQuery, user::UpdateUser,
    },
    repository::{
        auth::AuthRepository, icon::IconRepository, problem::ProblemRepository,
        session::SessionRepository, submission::SubmissionRepository, user::UserRepository,
    },
};

#[derive(Clone)]
pub struct UserService<
    UR: UserRepository,
    SR: SessionRepository,
    AR: AuthRepository,
    IR: IconRepository,
    PR: ProblemRepository,
    SubR: SubmissionRepository,
    C: MailClient,
> {
    user_repository: UR,
    session_repository: SR,
    auth_repository: AR,
    icon_repository: IR,
    problem_repository: PR,
    submission_repository: SubR,
    mail_client: C,
    mail_template_provider: Arc<dyn AuthMailTemplateProvider>,
}

impl<
    UR: UserRepository,
    SR: SessionRepository,
    AR: AuthRepository,
    IR: IconRepository,
    PR: ProblemRepository,
    SubR: SubmissionRepository,
    C: MailClient,
> UserService<UR, SR, AR, IR, PR, SubR, C>
{
    pub fn new(
        user_repository: UR,
        session_repository: SR,
        auth_repository: AR,
        icon_repository: IR,
        problem_repository: PR,
        submission_repository: SubR,
        mail_client: C,
    ) -> Self {
        Self {
            user_repository,
            session_repository,
            auth_repository,
            icon_repository,
            problem_repository,
            submission_repository,
            mail_client,
            mail_template_provider: Arc::new(DefaultAuthMailTemplateProvider::default()),
        }
    }
}

impl<
    UR: UserRepository,
    SR: SessionRepository,
    AR: AuthRepository,
    IR: IconRepository,
    PR: ProblemRepository,
    SubR: SubmissionRepository,
    C: MailClient,
> UserService<UR, SR, AR, IR, PR, SubR, C>
{
    pub async fn get_user(
        &self,
        display_id: String,
        session_id: Option<&str>,
    ) -> anyhow::Result<UserDto, UsecaseError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(UsecaseError::internal_server_error)?,
            None => None,
        };

        let display_id = display_id
            .parse::<i64>()
            .map_err(|_| UsecaseError::ValidateError)?;

        let user = self
            .user_repository
            .get_user_by_display_id(display_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let problem_query = ProblemGetQuery {
            user_id: user_id,
            limit: 50,
            offset: 0,
            order_by: domain::model::problem::ProblemOrderBy::CreatedAtDesc,
            user_query: Some(display_id),
            user_name: None,
        };

        let problem_count = self
            .problem_repository
            .get_problems_by_query_count(problem_query.clone())
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let problems = self
            .problem_repository
            .get_problems_by_query(problem_query)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let submission_query = SubmissionGetQuery {
            user_id: user_id,
            limit: 50,
            offset: 0,
            judge_status: None,
            language_id: None,
            user_name: None,
            user_query: Some(display_id),
            order_by: domain::model::submission::SubmissionOrderBy::SubmittedAtDesc,
            problem_id: None,
        };

        let submission_count = self
            .submission_repository
            .get_submissions_count_by_query(submission_query.clone())
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let submissions = self
            .submission_repository
            .get_submissions_by_query(submission_query)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(UserDto::new(
            user,
            NormalProblemsDto {
                total: problem_count,
                problems: problems.into_iter().map(|p| p.into()).collect(),
            },
            SubmissionsDto {
                total: submission_count,
                submissions: submissions.into_iter().map(|s| s.into()).collect(),
            },
        ))
    }

    pub async fn get_me(&self, session_id: &str) -> anyhow::Result<UserMeDto, UsecaseError> {
        let user_id = self
            .session_repository
            .get_display_id_by_session_id(session_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::Unauthorized)?;

        let user = self
            .user_repository
            .get_user_by_display_id(user_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let problem_query = ProblemGetQuery {
            user_id: Some(user_id),
            limit: 50,
            offset: 0,
            order_by: domain::model::problem::ProblemOrderBy::CreatedAtDesc,
            user_query: Some(user_id),
            user_name: None,
        };

        let problem_count = self
            .problem_repository
            .get_problems_by_query_count(problem_query.clone())
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let problems = self
            .problem_repository
            .get_problems_by_query(problem_query)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let submission_query = SubmissionGetQuery {
            user_id: Some(user_id),
            limit: 50,
            offset: 0,
            judge_status: None,
            language_id: None,
            user_name: None,
            user_query: Some(user_id),
            order_by: domain::model::submission::SubmissionOrderBy::SubmittedAtDesc,
            problem_id: None,
        };

        let submission_count = self
            .submission_repository
            .get_submissions_count_by_query(submission_query.clone())
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let submissions = self
            .submission_repository
            .get_submissions_by_query(submission_query)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let authentication = self
            .auth_repository
            .get_authentication_by_user_id(user.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(UserMeDto::new(
            user,
            NormalProblemsDto {
                total: problem_count,
                problems: problems.into_iter().map(|p| p.into()).collect(),
            },
            SubmissionsDto {
                total: submission_count,
                submissions: submissions.into_iter().map(|s| s.into()).collect(),
            },
            authentication,
        ))
    }

    pub async fn update_me(
        &self,
        session_id: &str,
        body: UpdateUserData,
    ) -> anyhow::Result<UserMeDto, UsecaseError> {
        body.validate().map_err(|_| UsecaseError::ValidateError)?;

        let user_id = self
            .session_repository
            .get_display_id_by_session_id(session_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::Unauthorized)?;

        let user = self
            .user_repository
            .get_user_by_display_id(user_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or_else(|| UsecaseError::internal_server_error_msg("user not found by display_id in update_me"))?;

        let icon_id = match body.icon {
            Some(icon) => {
                let binary_data = BASE64_STANDARD
                    .decode(icon)
                    .map_err(|_| UsecaseError::ValidateError)?;

                let mime_type = infer::get(&binary_data)
                    .ok_or(UsecaseError::ValidateError)?
                    .mime_type();

                if !mime_type.starts_with("image/") {
                    return Err(UsecaseError::ValidateError);
                }

                if binary_data.len() > 256 * 1024 {
                    return Err(UsecaseError::ValidateError);
                }

                if let Some(old_icon_id) = &user.icon_id {
                    self.icon_repository
                        .delete_icon(old_icon_id.to_owned())
                        .await
                        .map_err(UsecaseError::internal_server_error)?;
                }

                let uuid = uuid::Uuid::now_v7();

                let icon = domain::model::icon::Icon {
                    id: uuid,
                    content_type: mime_type.to_string(),
                    icon: binary_data,
                };

                self.icon_repository
                    .create_icon(icon)
                    .await
                    .map_err(UsecaseError::internal_server_error)?;

                Some(uuid)
            }
            None => None,
        };

        self.user_repository
            .update_user(
                user_id,
                UpdateUser {
                    user_name: body.user_name.unwrap_or(user.name),
                    icon_id: icon_id.or(user.icon_id),
                    github_id: body.github_id.or(user.github_id),
                    x_id: body.x_id.or(user.x_id),
                    self_introduction: body.self_introduction.unwrap_or(user.self_introduction),
                },
            )
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let new_user = self
            .user_repository
            .get_user_by_display_id(user_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or_else(|| UsecaseError::internal_server_error_msg("user not found by display_id after update in update_me"))?;

        let problem_query = ProblemGetQuery {
            user_id: Some(user_id),
            limit: 50,
            offset: 0,
            order_by: domain::model::problem::ProblemOrderBy::CreatedAtDesc,
            user_query: Some(user_id),
            user_name: None,
        };
        let problem_count = self
            .problem_repository
            .get_problems_by_query_count(problem_query.clone())
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let problems = self
            .problem_repository
            .get_problems_by_query(problem_query)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let submission_query = SubmissionGetQuery {
            user_id: Some(user_id),
            limit: 50,
            offset: 0,
            judge_status: None,
            language_id: None,
            user_name: None,
            user_query: Some(user_id),
            order_by: domain::model::submission::SubmissionOrderBy::SubmittedAtDesc,
            problem_id: None,
        };
        let submission_count = self
            .submission_repository
            .get_submissions_count_by_query(submission_query.clone())
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let submissions = self
            .submission_repository
            .get_submissions_by_query(submission_query)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let authentication = self
            .auth_repository
            .get_authentication_by_user_id(new_user.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(UserMeDto::new(
            new_user,
            NormalProblemsDto {
                total: problem_count,
                problems: problems.into_iter().map(|p| p.into()).collect(),
            },
            SubmissionsDto {
                total: submission_count,
                submissions: submissions.into_iter().map(|s| s.into()).collect(),
            },
            authentication,
        ))
    }

    pub async fn update_email(
        &self,
        session_id: &str,
        email: String,
    ) -> anyhow::Result<(), UsecaseError> {
        let user_address = email
            .parse::<Address>()
            .map_err(|_| UsecaseError::ValidateError)?;

        let display_id = self
            .session_repository
            .get_display_id_by_session_id(session_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::Unauthorized)?;

        if self
            .auth_repository
            .is_exist_email(&email)
            .await
            .map_err(UsecaseError::internal_server_error)?
        {
            return Err(UsecaseError::ValidateError);
        }

        let encode_key = std::env::var("JWT_SECRET_KEY").unwrap();
        let encrypt_key = std::env::var("JWT_PAYLOAD_ENCRYPT_SECRET_KEY").unwrap();

        let jwt = AuthToken::encode_email_update_jwt(display_id, &email, &encode_key, &encrypt_key)
            .map_err(UsecaseError::internal_server_error)?;

        let mail_content = self.mail_template_provider.change_email_request(&jwt);

        self.mail_client
            .send_mail(user_address, &mail_content.subject, &mail_content.body)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(())
    }

    pub async fn update_password(
        &self,
        session_id: &str,
        data: UpdatePasswordData,
    ) -> anyhow::Result<(), UsecaseError> {
        data.validate().map_err(|_| UsecaseError::ValidateError)?;

        let user_id = self
            .session_repository
            .get_user_id_by_session_id(session_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::Unauthorized)?;

        match self
            .auth_repository
            .verify_user_password(user_id, &data.old_password)
            .await
        {
            Ok(true) => {
                self.auth_repository
                    .update_user_password(user_id, &data.new_password)
                    .await
                    .map_err(UsecaseError::internal_server_error)?;
                Ok(())
            }
            Ok(false) => Err(UsecaseError::Unauthorized),
            Err(_) => Err(UsecaseError::internal_server_error_msg("verify_user_password failed unexpectedly")),
        }
    }
}
