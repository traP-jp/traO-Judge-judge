use axum::async_trait;

use crate::model::language::Language;

#[async_trait]
pub trait LanguageRepository {
    async fn get_languages(&self) -> anyhow::Result<Vec<Language>>;
    async fn language_to_id(&self, language: String) -> anyhow::Result<Option<i32>>;
}
