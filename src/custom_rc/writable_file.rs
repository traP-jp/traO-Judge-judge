use super::entity::file_entity::*;
use super::readonly_file::ReadonlyFile;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;

pub struct WritableFile {
    pub path: PathBuf,
    pub entity: WritableFileEntity,
}

impl WritableFile {
    pub fn new(path: PathBuf, entity: WritableFileEntity) -> Result<Self> {
        let target_path = match &entity {
            WritableFileEntity::TextFile(file) => file.path.clone(),
            WritableFileEntity::Directory(dir) => dir.path.clone(),
        };
        std::os::unix::fs::symlink(&target_path, &path).with_context(|| {
            format!(
                "Failed to create symlink from {:?} to {:?}",
                target_path, path
            )
        })?;
        Ok(Self { path, entity })
    }
}

impl super::File for WritableFile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl super::WritableFile<ReadonlyFile> for WritableFile {
    async fn to_readonly(self) -> Result<ReadonlyFile> {
        ReadonlyFile::new(
            self.path.clone(),
            ReadonlyFileEntity::from(match self.entity {
                WritableFileEntity::TextFile(file) => ReadonlyFileEntity::TextFile(Arc::new(file)),
                WritableFileEntity::Directory(dir) => ReadonlyFileEntity::Directory(Arc::new(dir)),
            }),
        )
        .context("Failed to create readonly file")
    }
}
