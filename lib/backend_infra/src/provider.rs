use std::path::PathBuf;

use async_sqlx_session::MySqlSessionStore;
use judge_core::logic::judge_service_impl::JudgeServiceImpl;
use judge_infra_mock::job_service::{job_service as mock_job_service, tokens as mock_tokens};
use judge_infra_mock::multi_proc_problem_registry::{
    registry_client::RegistryClient, registry_server::RegistryServer,
};
use sqlx::{
    MySql, MySqlPool, Pool, migrate,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
};

use crate::repository::{
    dep_name::DepNameRepositoryImpl, editorial::EditorialRepositoryImpl, icon::IconRepositoryImpl,
    language::LanguageRepositoryImpl, procedure::ProcedureRepositoryImpl,
};

use super::{
    external::mail::MailClientImpl,
    repository::{
        auth::AuthRepositoryImpl, problem::ProblemRepositoryImpl, session::SessionRepositoryImpl,
        submission::SubmissionRepositoryImpl, testcase::TestcaseRepositoryImpl,
        user::UserRepositoryImpl,
    },
};

#[derive(Clone, Debug)]
pub struct Provider {
    pool: MySqlPool,
    session_store: MySqlSessionStore,
    bcrypt_cost: u32,
    temp_dir: PathBuf,
}

impl Provider {
    pub async fn new() -> anyhow::Result<Self> {
        let options = get_option_from_env()?;

        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;

        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("user_sessions");

        migrate!("./migrations").run(&pool).await?;
        session_store.migrate().await?;

        let temp_dir = PathBuf::from("./tempdir");
        if !temp_dir.exists() {
            std::fs::create_dir_all(&temp_dir)?;
        }

        Ok(Self {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
            temp_dir,
        })
    }

    pub async fn create_by_pool(pool: Pool<MySql>) -> anyhow::Result<Self> {
        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("user_sessions");

        migrate!("./migrations").run(&pool).await?;
        session_store.migrate().await?;

        let temp_dir = PathBuf::from("./tempdir");
        if !temp_dir.exists() {
            std::fs::create_dir_all(&temp_dir)?;
        }

        Ok(Self {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
            temp_dir,
        })
    }

    pub fn provide_auth_repository(&self) -> AuthRepositoryImpl {
        AuthRepositoryImpl::new(self.bcrypt_cost, self.pool.clone())
    }

    pub fn provide_session_repository(&self) -> SessionRepositoryImpl {
        SessionRepositoryImpl::new(self.session_store.clone())
    }

    pub fn provide_user_repository(&self) -> UserRepositoryImpl {
        UserRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_icon_repository(&self) -> IconRepositoryImpl {
        IconRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_submission_repository(&self) -> SubmissionRepositoryImpl {
        SubmissionRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_problem_repository(&self) -> ProblemRepositoryImpl {
        ProblemRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_editorial_repository(&self) -> EditorialRepositoryImpl {
        EditorialRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_testcase_repository(&self) -> TestcaseRepositoryImpl {
        TestcaseRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_procedure_repository(&self) -> ProcedureRepositoryImpl {
        ProcedureRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_dep_name_repository(
        &self,
    ) -> crate::repository::dep_name::DepNameRepositoryImpl {
        DepNameRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_mail_client(&self) -> MailClientImpl {
        MailClientImpl::new().unwrap()
    }

    pub fn provide_problem_registry_client(&self) -> RegistryClient {
        RegistryClient::new(self.temp_dir.clone())
    }

    pub fn provide_problem_registry_server(&self) -> RegistryServer {
        RegistryServer::new(self.temp_dir.clone())
    }

    // Provide a local JudgeService using mock job service and multi-proc registry client
    pub fn provide_judge_service(
        &self,
    ) -> JudgeServiceImpl<
        mock_tokens::RegistrationToken,
        mock_tokens::OutcomeToken,
        mock_job_service::JobService<RegistryClient>,
    > {
        let host_temp_dir = self.temp_dir.clone();
        let container_temp_dir = PathBuf::from("/tmp/trao");
        let pr_client = self.provide_problem_registry_client();
        // Image name assumption; ensure this exists in your environment
        let image = std::env::var("TRAO_EXEC_IMAGE")
            .unwrap_or_else(|_| "traojudge/exec:latest".to_string());
        let job =
            mock_job_service::JobService::new(host_temp_dir, container_temp_dir, pr_client, image)
                .expect("Failed to init mock JobService");
        JudgeServiceImpl::new(job)
    }

    pub fn provide_language_repository(&self) -> LanguageRepositoryImpl {
        LanguageRepositoryImpl::new()
    }
}

fn get_option_from_env() -> anyhow::Result<MySqlConnectOptions> {
    let host = std::env::var("NS_MARIADB_HOSTNAME")?;
    let port = std::env::var("NS_MARIADB_PORT")?
        .parse()
        .map_err(|_| anyhow::anyhow!("DB_PORT must be a number"))?;
    let user = std::env::var("NS_MARIADB_USER")?;
    let password = std::env::var("NS_MARIADB_PASSWORD")?;
    let db_name = std::env::var("NS_MARIADB_DATABASE")?;

    Ok(MySqlConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&user)
        .password(&password)
        .database(&db_name))
}
