use crate::model::language::LanguageDto;

#[derive(Clone)]
pub struct LanguageService {}

impl LanguageService {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum LanguageError {
    InternalServerError,
}

impl LanguageService {
    pub async fn get_languages(&self) -> anyhow::Result<Vec<LanguageDto>, LanguageError> {
        let languages = vec![LanguageDto {
            name: "C".to_string(),
        }]; // todo
        Ok(languages)
    }
}
