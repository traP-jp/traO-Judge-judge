use crate::custom_rc::file_entity::DirectoryEntity;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

pub struct DirEntityFactory {
    pub path: PathBuf,
}

impl DirEntityFactory {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn get_dir_entity(&self) -> Result<DirectoryEntity> {
        Ok(DirectoryEntity::new(self.path.clone())?)
    }
}
