use base64::{prelude::BASE64_STANDARD, Engine};
use lettre::Address;

use crate::model::{
    problem::NormalProblemsDto,
    submission::SubmissionsDto,
    user::{UpdatePasswordData, UpdateUserData, UserDto},
};
use domain::{
    external::mail::MailClient,
    model::{
        jwt::EmailToken, problem::ProblemGetQuery, submission::SubmissionGetQuery, user::UpdateUser,
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
        }
    }
}

#[derive(Debug)]
pub enum UserError {
    ValidateError,
    Unauthorized,
    NotFound,
    InternalServerError,
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
        session_id: Option<String>,
    ) -> anyhow::Result<UserDto, UserError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(|_| UserError::InternalServerError)?,
            None => None,
        };

        let display_id = display_id
            .parse::<i64>()
            .map_err(|_| UserError::ValidateError)?;

        let user = self
            .user_repository
            .get_user_by_display_id(display_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::NotFound)?;

        let problem_query = ProblemGetQuery {
            user_id: user_id,
            limit: 50,
            offset: 0,
            order_by: domain::model::problem::ProblemOrderBy::CreatedAtDesc,
            user_query: Some(display_id),
        };

        let problem_count = self
            .problem_repository
            .get_problems_by_query_count(problem_query.clone())
            .await
            .map_err(|_| UserError::InternalServerError)?;
        let problems = self
            .problem_repository
            .get_problems_by_query(problem_query)
            .await
            .map_err(|_| UserError::InternalServerError)?;

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
            .map_err(|_| UserError::InternalServerError)?;
        let submissions = self
            .submission_repository
            .get_submissions_by_query(submission_query)
            .await
            .map_err(|_| UserError::InternalServerError)?;

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

    pub async fn get_me(&self, session_id: &str) -> anyhow::Result<UserDto, UserError> {
        let user_id = self
            .session_repository
            .get_display_id_by_session_id(session_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::Unauthorized)?;

        let user = self
            .user_repository
            .get_user_by_display_id(user_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::NotFound)?;

        let problem_query = ProblemGetQuery {
            user_id: Some(user_id),
            limit: 50,
            offset: 0,
            order_by: domain::model::problem::ProblemOrderBy::CreatedAtDesc,
            user_query: Some(user_id),
        };

        let problem_count = self
            .problem_repository
            .get_problems_by_query_count(problem_query.clone())
            .await
            .map_err(|_| UserError::InternalServerError)?;
        let problems = self
            .problem_repository
            .get_problems_by_query(problem_query)
            .await
            .map_err(|_| UserError::InternalServerError)?;

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
            .map_err(|_| UserError::InternalServerError)?;

        let submissions = self
            .submission_repository
            .get_submissions_by_query(submission_query)
            .await
            .map_err(|_| UserError::InternalServerError)?;

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

    pub async fn update_me(
        &self,
        session_id: &str,
        body: UpdateUserData,
    ) -> anyhow::Result<UserDto, UserError> {
        body.validate().map_err(|_| UserError::ValidateError)?;

        let user_id = self
            .session_repository
            .get_display_id_by_session_id(session_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::Unauthorized)?;

        let user = self
            .user_repository
            .get_user_by_display_id(user_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::InternalServerError)?;

        let icon_url = match body.icon {
            Some(icon) => {
                let binary_data = BASE64_STANDARD
                    .decode(icon)
                    .map_err(|_| UserError::ValidateError)?;

                let mime_type = infer::get(&binary_data)
                    .ok_or(UserError::ValidateError)?
                    .mime_type();

                if !mime_type.starts_with("image/") {
                    return Err(UserError::ValidateError);
                }

                if binary_data.len() > 256 * 1024 {
                    return Err(UserError::ValidateError);
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
                    .map_err(|_| UserError::InternalServerError)?;

                let icon_url = format!("{}/{}", std::env::var("ICON_HOST_URI").unwrap(), uuid);
                Some(icon_url)
            }
            None => None,
        };

        self.user_repository
            .update_user(
                user_id,
                UpdateUser {
                    user_name: body.user_name.unwrap_or(user.name),
                    icon_url,
                    x_link: body.x_link.or(user.x_link),
                    github_link: body.github_link.or(user.github_link),
                    self_introduction: body.self_introduction.unwrap_or(user.self_introduction),
                },
            )
            .await
            .map_err(|_| UserError::InternalServerError)?;

        let new_user = self
            .user_repository
            .get_user_by_display_id(user_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::InternalServerError)?;

        let problem_query = ProblemGetQuery {
            user_id: Some(user_id),
            limit: 50,
            offset: 0,
            order_by: domain::model::problem::ProblemOrderBy::CreatedAtDesc,
            user_query: Some(user_id),
        };
        let problem_count = self
            .problem_repository
            .get_problems_by_query_count(problem_query.clone())
            .await
            .map_err(|_| UserError::InternalServerError)?;
        let problems = self
            .problem_repository
            .get_problems_by_query(problem_query)
            .await
            .map_err(|_| UserError::InternalServerError)?;

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
            .map_err(|_| UserError::InternalServerError)?;
        let submissions = self
            .submission_repository
            .get_submissions_by_query(submission_query)
            .await
            .map_err(|_| UserError::InternalServerError)?;

        Ok(UserDto::new(
            new_user,
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

    pub async fn update_email(
        &self,
        session_id: &str,
        email: String,
    ) -> anyhow::Result<(), UserError> {
        let user_address = email
            .parse::<Address>()
            .map_err(|_| UserError::ValidateError)?;

        let display_id = self
            .session_repository
            .get_display_id_by_session_id(session_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::Unauthorized)?;

        if self
            .user_repository
            .is_exist_email(&email)
            .await
            .map_err(|_| UserError::InternalServerError)?
        {
            return Err(UserError::ValidateError);
        }

        let encode_key = std::env::var("JWT_SECRET_KEY").unwrap();
        let jwt = EmailToken::encode_email_update_jwt(display_id, &email, encode_key)
            .map_err(|_| UserError::InternalServerError)?;

        // todo
        let subject = "メールアドレス変更の確認";
        let message = format!(
            "以下のリンクをクリックして、メールアドレスの変更を確認してください。
    https://link/{jwt}"
        );

        self.mail_client
            .send_mail(user_address, subject, &message)
            .await
            .map_err(|_| UserError::InternalServerError)?;

        Ok(())
    }

    pub async fn update_password(
        &self,
        session_id: &str,
        data: UpdatePasswordData,
    ) -> anyhow::Result<(), UserError> {
        data.validate().map_err(|_| UserError::ValidateError)?;

        let user_id = self
            .session_repository
            .get_user_id_by_session_id(session_id)
            .await
            .map_err(|_| UserError::InternalServerError)?
            .ok_or(UserError::Unauthorized)?;

        match self
            .auth_repository
            .verify_user_password(user_id, &data.old_password)
            .await
        {
            Ok(true) => {
                self.auth_repository
                    .update_user_password(user_id, &data.new_password)
                    .await
                    .map_err(|_| UserError::InternalServerError)?;
                Ok(())
            }
            Ok(false) => Err(UserError::Unauthorized),
            Err(_) => Err(UserError::InternalServerError),
        }
    }
}
