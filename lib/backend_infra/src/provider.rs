use async_sqlx_session::MySqlSessionStore;
use aws_config::Region;
use aws_sdk_s3::{config::Credentials, Config};
use sqlx::{
    migrate,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySql, MySqlPool, Pool,
};

use super::{
    external::{mail::MailClientImpl, object_strage::ObjectStorageClientImpl},
    repository::{
        auth::AuthRepositoryImpl, problem::ProblemRepositoryImpl, session::SessionRepositoryImpl,
        submission::SubmissionRepositoryImpl, user::UserRepositoryImpl,
    },
};

#[derive(Clone, Debug)]
pub struct Provider {
    pool: MySqlPool,
    session_store: MySqlSessionStore,
    bcrypt_cost: u32,
    s3_client: aws_sdk_s3::Client,
    bucket_name: String,
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

        let config = get_config_from_env()?;
        let s3_client = aws_sdk_s3::Client::from_conf(config);
        let bucket_name = std::env::var("OBJECT_STORAGE_BUCKET")?;
        let _ = s3_client.create_bucket().bucket(&bucket_name).send().await;

        Ok(Self {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
            s3_client,
            bucket_name,
        })
    }

    pub async fn create_by_pool(pool: Pool<MySql>) -> anyhow::Result<Self> {
        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("user_sessions");

        migrate!("./migrations").run(&pool).await?;
        session_store.migrate().await?;

        let config = get_config_from_env()?;
        let s3_client = aws_sdk_s3::Client::from_conf(config);
        let bucket_name = std::env::var("OBJECT_STORAGE_BUCKET")?;
        let _ = s3_client.create_bucket().bucket(&bucket_name).send().await;

        Ok(Self {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
            s3_client,
            bucket_name,
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

    pub fn provide_submission_repository(&self) -> SubmissionRepositoryImpl {
        SubmissionRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_problem_repository(&self) -> ProblemRepositoryImpl {
        ProblemRepositoryImpl::new(self.pool.clone())
    }

    pub fn provide_mail_client(&self) -> MailClientImpl {
        MailClientImpl::new().unwrap()
    }

    pub fn provide_s3_client(&self) -> ObjectStorageClientImpl {
        ObjectStorageClientImpl::new(self.s3_client.clone(), self.bucket_name.clone())
    }
}

fn get_option_from_env() -> anyhow::Result<MySqlConnectOptions> {
    let host = std::env::var("DB_HOSTNAME")?;
    let port = std::env::var("DB_PORT")?
        .parse()
        .map_err(|_| anyhow::anyhow!("DB_PORT must be a number"))?;
    let user = std::env::var("DB_USERNAME")?;
    let password = std::env::var("DB_PASSWORD")?;
    let db_name = std::env::var("DB_DATABASE")?;

    Ok(MySqlConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&user)
        .password(&password)
        .database(&db_name))
}

fn get_config_from_env() -> anyhow::Result<Config> {
    let access_key = std::env::var("OBJECT_STORAGE_ACCESS_KEY")?;
    let secret_key = std::env::var("OBJECT_STORAGE_SECRET_KEY")?;
    let region = std::env::var("OBJECT_STORAGE_REGION")?;
    let endpoint = std::env::var("OBJECT_STORAGE_ENDPOINT")?;

    let credentials_provider = Credentials::new(access_key, secret_key, None, None, "static");
    let config = aws_sdk_s3::Config::builder()
        .behavior_version_latest()
        .credentials_provider(credentials_provider)
        .region(Region::new(region))
        .force_path_style(true)
        .endpoint_url(endpoint)
        .build();

    Ok(config)
}
