use axum::async_trait;

use crate::model::language::Language;

#[async_trait]
pub trait LanguageRepository {
    async fn get_language(&self) -> anyhow::Result<Vec<Language>>;
}
