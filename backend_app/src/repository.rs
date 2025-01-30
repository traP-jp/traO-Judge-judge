use async_sqlx_session::MySqlSessionStore;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

use super::Repository;

mod jwt;
mod normal_problems;
mod submissions;
mod user_password;
pub mod users;
mod users_session;

impl Repository {
    pub async fn connect() -> anyhow::Result<Self> {
        let options = get_option_from_env()?;

        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;

        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("user_sessions");

        Ok(Self {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
        })
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        self.session_store.migrate().await?;

        Ok(())
    }

    pub async fn create_by_pool(pool: sqlx::MySqlPool) -> anyhow::Result<Self> {
        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("user_sessions");
        session_store.migrate().await?;

        Ok(Self {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
        })
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
