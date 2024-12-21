use super::entity::file_entity::*;
use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Clone)]
pub struct ReadonlyFile {
    pub path: PathBuf,
    pub _entity: ReadonlyFileEntity,
}

impl ReadonlyFile {
    pub fn new(path: PathBuf, entity: ReadonlyFileEntity) -> Result<Self> {
        let target_path = match &entity {
            ReadonlyFileEntity::TextFile(file) => file.path.clone(),
            ReadonlyFileEntity::Directory(dir) => dir.path.clone(),
        };
        std::os::unix::fs::symlink(&target_path, &path).with_context(|| {
            format!(
                "Failed to create symlink from {:?} to {:?}",
                target_path, path
            )
        })?;
        Ok(Self {
            path,
            _entity: entity,
        })
    }
}

impl super::File for ReadonlyFile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl super::ReadonlyFile for ReadonlyFile {}
