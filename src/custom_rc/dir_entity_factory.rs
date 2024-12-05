use crate::custom_rc::file_entity::DirectoryEntity;
use anyhow::Result;
use std::path::PathBuf;

pub struct DirEntityFactory {
    pub path: PathBuf,
}

impl DirEntityFactory {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub async fn get_dir_entity(&self) -> Result<DirectoryEntity> {
        DirectoryEntity::new(self.path.clone()).await
    }
}
