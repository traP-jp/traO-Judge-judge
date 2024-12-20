use super::entity::file_entity::*;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Result, Context};

#[derive(Clone)]
pub struct ReadonlyFile {
    pub path: PathBuf,
    pub entity: ReadonlyFileEntity,
}

impl ReadonlyFile {
    pub fn new(path: PathBuf, entity: ReadonlyFileEntity) -> Result<Self> {
        let target_path = match &entity {
            ReadonlyFileEntity::TextFile(file) => file.path.clone(),
            ReadonlyFileEntity::Directory(dir) => dir.path.clone(),
        };
        std::os::unix::fs::symlink(&target_path, &path)
            .with_context(|| format!("Failed to create symlink from {:?} to {:?}", target_path, path))?;
        Ok(Self { path, entity })
    }
}

impl super::File for ReadonlyFile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl super::ReadonlyFile for ReadonlyFile {}
