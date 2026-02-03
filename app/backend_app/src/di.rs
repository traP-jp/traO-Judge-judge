use crate::config::AppMode;
use back_judge_grpc::client::RemoteJudgeServiceClient;
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
use judge_core::{
    logic::judge_service_impl::JudgeServiceImpl,
    model::{
        identifiers::ResourceId,
        judge::{JudgeRequest, JudgeResponse, JudgeService},
        problem_registry::{
            ProblemRegistryClient, ProblemRegistryServer, RegistrationError, RemovalError,
            ResourceFetchError,
        },
    },
};
use judge_infra_mock::job_service::{job_service as mock_job_service, tokens as mock_tokens};
use judge_infra_mock::multi_proc_problem_registry::{
    registry_client::RegistryClient as MockRegistryClient,
    registry_server::RegistryServer as MockRegistryServer,
};
use problem_registry::{
    client::ProblemRegistryClient as ProdProblemRegistryClient,
    server::ProblemRegistryServer as ProdProblemRegistryServer,
};
use usecase::service::{
    auth::AuthenticationService, editorial::EditorialService, github_oauth2::GitHubOAuth2Service,
    google_oauth2::GoogleOAuth2Service, icon::IconService, language::LanguageService,
    problem::ProblemService, submission::SubmissionService, testcase::TestcaseService,
    traq_oauth2::TraqOAuth2Service, user::UserService,
};

type DevJudgeService = JudgeServiceImpl<
    mock_tokens::RegistrationToken,
    mock_tokens::OutcomeToken,
    mock_job_service::JobService<MockRegistryClient>,
>;

#[derive(Clone)]
enum RegistryServerRuntime {
    Dev(MockRegistryServer),
    Prod(ProdProblemRegistryServer),
}

#[axum::async_trait]
impl ProblemRegistryServer for RegistryServerRuntime {
    async fn register(
        &self,
        resource_id: ResourceId,
        content: String,
    ) -> Result<(), RegistrationError> {
        match self {
            RegistryServerRuntime::Dev(inner) => inner.register(resource_id, content).await,
            RegistryServerRuntime::Prod(inner) => inner.register(resource_id, content).await,
        }
    }

    async fn remove(&self, resource_id: ResourceId) -> Result<(), RemovalError> {
        match self {
            RegistryServerRuntime::Dev(inner) => inner.remove(resource_id).await,
            RegistryServerRuntime::Prod(inner) => inner.remove(resource_id).await,
        }
    }
}

#[derive(Clone)]
enum RegistryClientRuntime {
    Dev(MockRegistryClient),
    Prod(ProdProblemRegistryClient),
}

#[axum::async_trait]
impl ProblemRegistryClient for RegistryClientRuntime {
    async fn fetch(&self, resource_id: ResourceId) -> Result<String, ResourceFetchError> {
        match self {
            RegistryClientRuntime::Dev(inner) => inner.fetch(resource_id).await,
            RegistryClientRuntime::Prod(inner) => inner.fetch(resource_id).await,
        }
    }
}

#[derive(Clone)]
enum JudgeServiceRuntime {
    Dev(DevJudgeService),
    Prod(RemoteJudgeServiceClient),
}

#[axum::async_trait]
impl JudgeService for JudgeServiceRuntime {
    async fn judge(&self, request: JudgeRequest) -> JudgeResponse {
        match self {
            JudgeServiceRuntime::Dev(inner) => inner.judge(request).await,
            JudgeServiceRuntime::Prod(inner) => inner.judge(request).await,
        }
    }
}

type RegistryServerImpl = RegistryServerRuntime;
type RegistryClientImpl = RegistryClientRuntime;
type JudgeSvcImpl = JudgeServiceRuntime;

type AuthSvc = AuthenticationService<
    AuthRepositoryImpl,
    UserRepositoryImpl,
    SessionRepositoryImpl,
    MailClientImpl,
>;
type ProblemSvc = ProblemService<
    ProblemRepositoryImpl,
    UserRepositoryImpl,
    SessionRepositoryImpl,
    TestcaseRepositoryImpl,
    ProcedureRepositoryImpl,
    RegistryServerImpl,
    DepNameRepositoryImpl,
>;
type UserSvc = UserService<
    UserRepositoryImpl,
    SessionRepositoryImpl,
    AuthRepositoryImpl,
    IconRepositoryImpl,
    ProblemRepositoryImpl,
    SubmissionRepositoryImpl,
    MailClientImpl,
>;
type IconSvc = IconService<IconRepositoryImpl>;
type SubmissionSvc = SubmissionService<
    SessionRepositoryImpl,
    UserRepositoryImpl,
    SubmissionRepositoryImpl,
    ProblemRepositoryImpl,
    ProcedureRepositoryImpl,
    TestcaseRepositoryImpl,
    LanguageRepositoryImpl,
    DepNameRepositoryImpl,
    JudgeSvcImpl,
>;
type EditorialSvc =
    EditorialService<SessionRepositoryImpl, EditorialRepositoryImpl, ProblemRepositoryImpl>;
type TestcaseSvc = TestcaseService<
    ProblemRepositoryImpl,
    SessionRepositoryImpl,
    TestcaseRepositoryImpl,
    ProcedureRepositoryImpl,
    RegistryClientImpl,
    RegistryServerImpl,
    DepNameRepositoryImpl,
>;
type LanguageSvc = LanguageService<LanguageRepositoryImpl>;
type GoogleOAuth2Svc =
    GoogleOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl>;
type GitHubOAuth2Svc =
    GitHubOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl>;
type TraqOAuth2Svc =
    TraqOAuth2Service<AuthRepositoryImpl, SessionRepositoryImpl, UserRepositoryImpl>;

#[derive(Clone)]
pub struct DiContainer {
    auth_service: AuthSvc,
    problem_service: ProblemSvc,
    user_service: UserSvc,
    icon_service: IconSvc,
    submission_service: std::sync::Arc<SubmissionSvc>,
    editorial_service: EditorialSvc,
    testcase_service: TestcaseSvc,
    language_service: LanguageSvc,
    google_oauth2_service: GoogleOAuth2Svc,
    github_oauth2_service: GitHubOAuth2Svc,
    traq_oauth2_service: TraqOAuth2Svc,
}

impl DiContainer {
    pub async fn new(provider: Provider) -> Self {
        let mode = AppMode::from_env();

        let pr_server: RegistryServerImpl = match mode {
            AppMode::Dev => RegistryServerRuntime::Dev(provider.provide_problem_registry_server()),
            AppMode::Prod => RegistryServerRuntime::Prod(ProdProblemRegistryServer::new().await),
        };
        let pr_client: RegistryClientImpl = match mode {
            AppMode::Dev => RegistryClientRuntime::Dev(provider.provide_problem_registry_client()),
            AppMode::Prod => RegistryClientRuntime::Prod(ProdProblemRegistryClient::new().await),
        };

        let judge_service: JudgeSvcImpl = match mode {
            AppMode::Dev => JudgeServiceRuntime::Dev(provider.provide_judge_service()),
            AppMode::Prod => {
                let uri = std::env::var("JUDGE_SERVICE_GRPC_URI")
                    .unwrap_or_else(|_| "http://localhost:50051".to_string());
                let client = RemoteJudgeServiceClient::new(&uri)
                    .await
                    .expect("Failed to create RemoteJudgeServiceClient");
                JudgeServiceRuntime::Prod(client)
            }
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
                provider.provide_user_repository(),
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
                pr_client,
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

    pub fn user_service(&self) -> &UserSvc {
        &self.user_service
    }

    pub fn auth_service(&self) -> &AuthSvc {
        &self.auth_service
    }

    pub fn icon_service(&self) -> &IconSvc {
        &self.icon_service
    }

    pub fn submission_service(&self) -> &std::sync::Arc<SubmissionSvc> {
        &self.submission_service
    }

    pub fn problem_service(&self) -> &ProblemSvc {
        &self.problem_service
    }

    pub fn editorial_service(&self) -> &EditorialSvc {
        &self.editorial_service
    }

    pub fn testcase_service(&self) -> &TestcaseSvc {
        &self.testcase_service
    }

    pub fn language_service(&self) -> &LanguageSvc {
        &self.language_service
    }

    pub fn google_oauth2_service(&self) -> &GoogleOAuth2Svc {
        &self.google_oauth2_service
    }

    pub fn github_oauth2_service(&self) -> &GitHubOAuth2Svc {
        &self.github_oauth2_service
    }

    pub fn traq_oauth2_service(&self) -> &TraqOAuth2Svc {
        &self.traq_oauth2_service
    }
}
