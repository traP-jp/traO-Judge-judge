use infra::{
    external::mail::MailClientImpl,
    provider::Provider,
    repository::{
        auth::AuthRepositoryImpl, dep_name::DepNameRepositoryImpl,
        editorial::EditorialRepositoryImpl, icon::IconRepositoryImpl,
        problem::ProblemRepositoryImpl, procedure::ProcedureRepositoryImpl,
        session::SessionRepositoryImpl, submission::SubmissionRepositoryImpl,
        testcase::TestcaseRepositoryImpl, user::UserRepositoryImpl,
    },
};
use judge_core::logic::judge_service_impl::JudgeServiceImpl;
use judge_infra_mock::job_service::{job_service as mock_job_service, tokens as mock_tokens};
use judge_infra_mock::multi_proc_problem_registry::{
    registry_client::RegistryClient, registry_server::RegistryServer,
};
use usecase::service::{
    auth::AuthenticationService, editorial::EditorialService, icon::IconService,
    problem::ProblemService, submission::SubmissionService, testcase::TestcaseService,
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
    problem_service: ProblemService<
        ProblemRepositoryImpl,
        UserRepositoryImpl,
        SessionRepositoryImpl,
        TestcaseRepositoryImpl,
    >,
    user_service: UserService<
        UserRepositoryImpl,
        SessionRepositoryImpl,
        AuthRepositoryImpl,
        IconRepositoryImpl,
        ProblemRepositoryImpl,
        SubmissionRepositoryImpl,
        MailClientImpl,
    >,
    icon_service: IconService<IconRepositoryImpl>,
    submission_service: SubmissionService<
        SessionRepositoryImpl,
        SubmissionRepositoryImpl,
        ProblemRepositoryImpl,
        ProcedureRepositoryImpl,
        TestcaseRepositoryImpl,
        UserRepositoryImpl,
        DepNameRepositoryImpl,
        JudgeServiceImpl<
            mock_tokens::RegistrationToken,
            mock_tokens::OutcomeToken,
            mock_job_service::JobService<RegistryClient>,
        >,
    >,
    editorial_service:
        EditorialService<SessionRepositoryImpl, EditorialRepositoryImpl, ProblemRepositoryImpl>,
    testcase_service: TestcaseService<
        ProblemRepositoryImpl,
        SessionRepositoryImpl,
        TestcaseRepositoryImpl,
        ProcedureRepositoryImpl,
        RegistryClient, // mock
        RegistryServer, // mock
        DepNameRepositoryImpl,
    >,
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
                provider.provide_user_repository(),
                provider.provide_session_repository(),
                provider.provide_testcase_repository(),
            ),
            user_service: UserService::new(
                provider.provide_user_repository(),
                provider.provide_session_repository(),
                provider.provide_auth_repository(),
                provider.provide_icon_repository(),
                provider.provide_problem_repository(),
                provider.provide_submission_repository(),
                provider.provide_mail_client(),
            ),
            icon_service: IconService::new(provider.provide_icon_repository()),
            submission_service: SubmissionService::new(
                provider.provide_session_repository(),
                provider.provide_submission_repository(),
                provider.provide_problem_repository(),
                provider.provide_procedure_repository(),
                provider.provide_testcase_repository(),
                provider.provide_user_repository(),
                provider.provide_dep_name_repository(),
                provider.provide_judge_service(),
            ),
            editorial_service: EditorialService::new(
                provider.provide_session_repository(),
                provider.provide_editorial_repository(),
                provider.provide_problem_repository(),
            ),
            testcase_service: TestcaseService::new(
                provider.provide_problem_repository(),
                provider.provide_session_repository(),
                provider.provide_testcase_repository(),
                provider.provide_procedure_repository(),
                provider.provide_problem_registry_client(),
                provider.provide_problem_registry_server(),
                provider.provide_dep_name_repository(),
            ),
        }
    }

    pub fn user_service(
        &self,
    ) -> &UserService<
        UserRepositoryImpl,
        SessionRepositoryImpl,
        AuthRepositoryImpl,
        IconRepositoryImpl,
        ProblemRepositoryImpl,
        SubmissionRepositoryImpl,
        MailClientImpl,
    > {
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

    pub fn icon_service(&self) -> &IconService<IconRepositoryImpl> {
        &self.icon_service
    }

    pub fn submission_service(
        &self,
    ) -> &SubmissionService<
        SessionRepositoryImpl,
        SubmissionRepositoryImpl,
        ProblemRepositoryImpl,
        ProcedureRepositoryImpl,
        TestcaseRepositoryImpl,
        UserRepositoryImpl,
        DepNameRepositoryImpl,
        JudgeServiceImpl<
            mock_tokens::RegistrationToken,
            mock_tokens::OutcomeToken,
            mock_job_service::JobService<RegistryClient>,
        >,
    > {
        &self.submission_service
    }

    pub fn problem_service(
        &self,
    ) -> &ProblemService<
        ProblemRepositoryImpl,
        UserRepositoryImpl,
        SessionRepositoryImpl,
        TestcaseRepositoryImpl,
    > {
        &self.problem_service
    }

    pub fn editorial_service(
        &self,
    ) -> &EditorialService<SessionRepositoryImpl, EditorialRepositoryImpl, ProblemRepositoryImpl>
    {
        &self.editorial_service
    }

    pub fn testcase_service(
        &self,
    ) -> &TestcaseService<
        ProblemRepositoryImpl,
        SessionRepositoryImpl,
        TestcaseRepositoryImpl,
        ProcedureRepositoryImpl,
        RegistryClient, // mock
        RegistryServer, // mock
        DepNameRepositoryImpl,
    > {
        &self.testcase_service
    }
}
