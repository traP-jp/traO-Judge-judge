use super::entity::file_entity::*;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Result, Context};

#[derive(Clone)]
pub struct ReadonlyFile {
    pub path: PathBuf,
    pub entity: Arc<FileEntity>,
}

impl ReadonlyFile {
    pub fn new(path: PathBuf, entity: Arc<FileEntity>) -> Result<Self> {
        let target_path = match &*entity {
            FileEntity::TextFile(_) => path,
            FileEntity::Directory(_) => path,
        };
        std::os::unix::fs::symlink(&target_path, &path)
            .with_context(|| format!("Failed to create symlink from {:?} to {:?}", target_path, path))?;
        Ok(Self { path, entity })
    }
}

impl super::ReadonlyFile for ReadonlyFile {}
