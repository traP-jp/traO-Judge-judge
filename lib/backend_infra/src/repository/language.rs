use axum::async_trait;
use domain::{model::language::Language, repository::language::LanguageRepository};

use crate::model::language::LanguagesRow;

use std::{env, fs};

#[derive(Clone)]
pub struct LanguageRepositoryImpl {
    pub languages: Vec<Language>,
}

impl LanguageRepositoryImpl {
    pub fn new() -> Self {
        match load_languages() {
            Ok(languages) => LanguageRepositoryImpl { languages },
            Err(_) => LanguageRepositoryImpl {
                languages: Vec::new(),
            },
        }
    }
}

#[async_trait]
impl LanguageRepository for LanguageRepositoryImpl {
    async fn get_languages(&self) -> anyhow::Result<Vec<Language>> {
        Ok(self.languages.clone())
    }
}

fn load_languages() -> anyhow::Result<Vec<Language>> {
    let var_name = "LANGUAGES_PATH";
    let path = env::var(var_name)?;
    let s = fs::read_to_string(path)?;
    let languages: LanguagesRow = serde_json::from_str(&s)?;
    Ok(languages.languages.into_iter().map(|l| l.into()).collect())
}
