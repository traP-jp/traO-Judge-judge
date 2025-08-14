use axum::async_trait;
use domain::{model::language::Language, repository::language::LanguageRepository};

use crate::model::language::LanguageRow;

#[derive(Clone)]
pub struct LanguageRepositoryImpl {}

impl LanguageRepositoryImpl {
    pub fn new() -> Self {
        LanguageRepositoryImpl {}
    }
}

#[async_trait]
impl LanguageRepository for LanguageRepositoryImpl {
    async fn get_language(&self) -> anyhow::Result<Vec<Language>> {
        let var_name = "LANGUAGES";
        let s = std::env::var(var_name)?;
        let languages: Vec<LanguageRow> = serde_json::from_str(&s)?;
        Ok(languages
            .into_iter()
            .map(|language| language.into())
            .collect())
    }
}
