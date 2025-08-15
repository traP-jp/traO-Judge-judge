use crate::model::language::{LanguageDto, LanguagesDto};
use std::{env, fs};

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
        let var_name = "LANGUAGES_PATH";
        let path = env::var(var_name).map_err(|_| LanguageError::InternalServerError)?;
        let s = fs::read_to_string(path).map_err(|_| LanguageError::InternalServerError)?;
        let languages: LanguagesDto =
            serde_json::from_str(&s).map_err(|_| LanguageError::InternalServerError)?;

        Ok(languages.languages)
    }
}
