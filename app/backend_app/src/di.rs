use infra::{
    external::mail::MailClientImpl,
    provider::Provider,
    repository::{
        auth::AuthRepositoryImpl, dep_name::DepNameRepositoryImpl,
        editorial::EditorialRepositoryImpl, icon::IconRepositoryImpl,
        language::LanguageRepositoryImpl, problem::ProblemRepositoryImpl,
        procedure::ProcedureRepositoryImpl, session::SessionRepositoryImpl,
        submission::SubmissionRepositoryImpl, testcase::TestcaseRepositoryImpl,
        user::UserRepositoryImpl,
    },
};
use judge_core::logic::judge_service_impl::JudgeServiceImpl;

#[cfg(feature = "dev")]
use judge_infra_mock::job_service::{job_service as mock_job_service, tokens as mock_tokens};
#[cfg(feature = "dev")]
use judge_infra_mock::multi_proc_problem_registry::{
    registry_client::RegistryClient as MockRegistryClient,
    registry_server::RegistryServer as MockRegistryServer,
};

#[cfg(feature = "prod")]
use job_service::{aws::AwsClient, grpc::GrpcClient, job_service::JobService as RealJobService};
#[cfg(feature = "prod")]
use problem_registry::{client::ProblemRegistryClient, server::ProblemRegistryServer};

#[cfg(feature = "dev")]
type RegistryServerImpl = MockRegistryServer;
#[cfg(feature = "dev")]
type RegistryClientImpl = MockRegistryClient;

#[cfg(feature = "prod")]
type RegistryServerImpl = ProblemRegistryServer;
#[cfg(feature = "prod")]
type RegistryClientImpl = ProblemRegistryClient;

use usecase::service::{
    auth::AuthenticationService, editorial::EditorialService, github_oauth2::GitHubOAuth2Service,
    google_oauth2::GoogleOAuth2Service, icon::IconService, language::LanguageService,
    problem::ProblemService, submission::SubmissionService, testcase::TestcaseService,
    traq_oauth2::TraqOAuth2Service, user::UserService,
};

#[cfg(feature = "dev")]
type JudgeSvcImpl = JudgeServiceImpl<
    mock_tokens::RegistrationToken,
    mock_tokens::OutcomeToken,
    mock_job_service::JobService<MockRegistryClient>,
>;
#[cfg(feature = "prod")]
type JudgeSvcImpl = JudgeServiceImpl<
    job_service::job_service::ReservationToken,
    job_service::job_service::OutcomeToken,
    RealJobService,
>;

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
        ProcedureRepositoryImpl,
        RegistryServerImpl,
        DepNameRepositoryImpl,
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
    submission_service: std::sync::Arc<
        SubmissionService<
            SessionRepositoryImpl,
            SubmissionRepositoryImpl,
            ProblemRepositoryImpl,
            ProcedureRepositoryImpl,
            TestcaseRepositoryImpl,
            LanguageRepositoryImpl,
            DepNameRepositoryImpl,
            JudgeSvcImpl,
        >,
    >,
    editorial_service:
        EditorialService<SessionRepositoryImpl, EditorialRepositoryImpl, ProblemRepositoryImpl>,
    testcase_service: TestcaseService<
        ProblemRepositoryImpl,
        SessionRepositoryImpl,
        TestcaseRepositoryImpl,
        ProcedureRepositoryImpl,
        RegistryClientImpl,
        RegistryServerImpl,
        DepNameRepositoryImpl,
    >,
    language_service: LanguageService<LanguageRepositoryImpl>,
    google_oauth2_service:
        GoogleOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl>,
    github_oauth2_service:
        GitHubOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl>,
    traq_oauth2_service:
        TraqOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl>,
}

impl DiContainer {
    pub async fn new(provider: Provider) -> Self {
        #[cfg(feature = "dev")]
        let pr_server: RegistryServerImpl = provider.provide_problem_registry_server();
        #[cfg(feature = "dev")]
        let pr_client: RegistryClientImpl = provider.provide_problem_registry_client();

        #[cfg(feature = "prod")]
        let pr_server: RegistryServerImpl = ProblemRegistryServer::new().await;
        #[cfg(feature = "prod")]
        let pr_client: RegistryClientImpl = ProblemRegistryClient::new().await;

        #[cfg(feature = "dev")]
        let judge_service: JudgeSvcImpl = provider.provide_judge_service();
        #[cfg(feature = "prod")]
        let judge_service: JudgeSvcImpl = {
            let aws_factory = || async move { AwsClient::new().await };
            let grpc_factory = |ip| async move { GrpcClient::new(ip).await };
            let pr_factory = || async move { ProblemRegistryClient::new().await };
            let job_service = RealJobService::new(aws_factory, grpc_factory, pr_factory);
            JudgeServiceImpl::new(job_service)
        };

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
                provider.provide_procedure_repository(),
                pr_server.clone(),
                provider.provide_dep_name_repository(),
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
            submission_service: std::sync::Arc::new(SubmissionService::new(
                provider.provide_session_repository(),
                provider.provide_submission_repository(),
                provider.provide_problem_repository(),
                provider.provide_procedure_repository(),
                provider.provide_testcase_repository(),
                provider.provide_language_repository(),
                provider.provide_dep_name_repository(),
                judge_service,
            )),
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
                pr_client.clone(),
                pr_server,
                provider.provide_dep_name_repository(),
            ),
            language_service: LanguageService::new(provider.provide_language_repository()),
            google_oauth2_service: GoogleOAuth2Service::new(
                provider.provide_auth_repository(),
                provider.provide_session_repository(),
                provider.provide_user_repository(),
            ),
            github_oauth2_service: GitHubOAuth2Service::new(
                provider.provide_auth_repository(),
                provider.provide_session_repository(),
                provider.provide_user_repository(),
            ),
            traq_oauth2_service: TraqOAuth2Service::new(
                provider.provide_auth_repository(),
                provider.provide_session_repository(),
                provider.provide_user_repository(),
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
    ) -> &std::sync::Arc<
        SubmissionService<
            SessionRepositoryImpl,
            SubmissionRepositoryImpl,
            ProblemRepositoryImpl,
            ProcedureRepositoryImpl,
            TestcaseRepositoryImpl,
            LanguageRepositoryImpl,
            DepNameRepositoryImpl,
            JudgeSvcImpl,
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
        ProcedureRepositoryImpl,
        RegistryServerImpl,
        DepNameRepositoryImpl,
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
        RegistryClientImpl,
        RegistryServerImpl,
        DepNameRepositoryImpl,
    > {
        &self.testcase_service
    }

    pub fn language_service(&self) -> &LanguageService<LanguageRepositoryImpl> {
        &self.language_service
    }

    pub fn google_oauth2_service(
        &self,
    ) -> &GoogleOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl> {
        &self.google_oauth2_service
    }

    pub fn github_oauth2_service(
        &self,
    ) -> &GitHubOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl> {
        &self.github_oauth2_service
    }

    pub fn traq_oauth2_service(
        &self,
    ) -> &TraqOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl> {
        &self.traq_oauth2_service
    }
}
