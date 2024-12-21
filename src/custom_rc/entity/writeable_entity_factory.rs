use super::file_entity::*;
use anyhow::Result;
use std::path::PathBuf;

pub struct WriteableEntityFactory {
    base_path: PathBuf,
}

impl WriteableEntityFactory {
    pub fn new(path: PathBuf) -> Self {
        Self { base_path: path }
    }

    pub async fn get_dir_entity(&self) -> Result<DirectoryEntity> {
        DirectoryEntity::new(self.base_path.join(uuid::Uuid::new_v4().to_string())).await
    }

    pub async fn get_text_file_entity(&self, content: &str) -> Result<TextFileEntity> {
        TextFileEntity::new(
            self.base_path.join(uuid::Uuid::new_v4().to_string()),
            content,
        )
        .await
    }
}
