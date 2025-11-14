use crate::model::{error::UsecaseError, language::LanguageDto};
use domain::repository::language::LanguageRepository;

#[derive(Clone)]
pub struct LanguageService<LR: LanguageRepository> {
    language_repository: LR,
}

impl<LR: LanguageRepository> LanguageService<LR> {
    pub fn new(language_repository: LR) -> Self {
        Self {
            language_repository,
        }
    }
}

impl<LR: LanguageRepository> LanguageService<LR> {
    pub async fn get_languages(&self) -> anyhow::Result<Vec<LanguageDto>, UsecaseError> {
        let languages = self
            .language_repository
            .get_languages()
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(languages.into_iter().map(|l| l.into()).collect())
    }
}
