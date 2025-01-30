use async_sqlx_session::MySqlSessionStore;
use sqlx::MySqlPool;

mod handler;
mod repository;
mod utils;

#[must_use]
#[derive(Clone)]
pub struct Repository {
    pool: MySqlPool,
    session_store: MySqlSessionStore,
    bcrypt_cost: u32,
}

pub fn make_router(app_state: Repository) -> axum::Router {
    handler::make_router(app_state)
}
