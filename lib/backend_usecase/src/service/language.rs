use crate::model::language::LanguageDto;
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

pub enum LanguageError {
    InternalServerError,
}

impl<LR: LanguageRepository> LanguageService<LR> {
    pub async fn get_languages(&self) -> anyhow::Result<Vec<LanguageDto>, LanguageError> {
        let languages = self
            .language_repository
            .get_language()
            .await
            .map_err(|_| LanguageError::InternalServerError)?;

        Ok(languages
            .into_iter()
            .map(|language| language.into())
            .collect())
    }
}
