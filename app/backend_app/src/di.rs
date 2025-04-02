use infra::{
    external::mail::MailClientImpl,
    provider::Provider,
    repository::{
        auth::AuthRepositoryImpl, problem::ProblemRepositoryImpl, session::SessionRepositoryImpl,
        submission::SubmissionRepositoryImpl, testcase::TestcaseRepositoryImpl,
        user::UserRepositoryImpl,
    },
};
use usecase::service::{
    auth::AuthenticationService, problem::ProblemService, submission::SubmissionService,
    user::UserService,
};

#[derive(Clone)]
pub struct DiContainer {
    auth_service: AuthenticationService<
        AuthRepositoryImpl,
        UserRepositoryImpl,
        SessionRepositoryImpl,
        MailClientImpl,
    >,
    problem_service:
        ProblemService<ProblemRepositoryImpl, SessionRepositoryImpl, TestcaseRepositoryImpl>,
    user_service:
        UserService<UserRepositoryImpl, SessionRepositoryImpl, AuthRepositoryImpl, MailClientImpl>,
    submission_service:
        SubmissionService<SessionRepositoryImpl, SubmissionRepositoryImpl, ProblemRepositoryImpl>,
}

impl DiContainer {
    pub async fn new(provider: Provider) -> Self {
        Self {
            auth_service: AuthenticationService::new(
                provider.provide_auth_repository(),
                provider.provide_user_repository(),
                provider.provide_session_repository(),
                provider.provide_mail_client(),
            ),
            problem_service: ProblemService::new(
                provider.provide_problem_repository(),
                provider.provide_session_repository(),
                provider.provide_testcase_repository(),
            ),
            user_service: UserService::new(
                provider.provide_user_repository(),
                provider.provide_session_repository(),
                provider.provide_auth_repository(),
                provider.provide_mail_client(),
            ),
            submission_service: SubmissionService::new(
                provider.provide_session_repository(),
                provider.provide_submission_repository(),
                provider.provide_problem_repository(),
            ),
        }
    }

    pub fn user_service(
        &self,
    ) -> &UserService<UserRepositoryImpl, SessionRepositoryImpl, AuthRepositoryImpl, MailClientImpl>
    {
        &self.user_service
    }

    pub fn auth_service(
        &self,
    ) -> &AuthenticationService<
        AuthRepositoryImpl,
        UserRepositoryImpl,
        SessionRepositoryImpl,
        MailClientImpl,
    > {
        &self.auth_service
    }

    pub fn submission_service(
        &self,
    ) -> &SubmissionService<SessionRepositoryImpl, SubmissionRepositoryImpl, ProblemRepositoryImpl>
    {
        &self.submission_service
    }

    pub fn problem_service(
        &self,
    ) -> &ProblemService<ProblemRepositoryImpl, SessionRepositoryImpl, TestcaseRepositoryImpl> {
        &self.problem_service
    }
}
