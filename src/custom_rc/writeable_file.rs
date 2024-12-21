use super::entity::file_entity::*;
use super::readonly_file::ReadonlyFile;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;

pub struct WriteableFile {
    pub path: PathBuf,
    pub entity: WriteableFileEntity,
}

impl WriteableFile {
    pub fn new(path: PathBuf, entity: WriteableFileEntity) -> Result<Self> {
        let target_path = match &entity {
            WriteableFileEntity::TextFile(file) => file.path.clone(),
            WriteableFileEntity::Directory(dir) => dir.path.clone(),
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

impl super::File for WriteableFile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl super::WriteableFile<ReadonlyFile> for WriteableFile {
    async fn to_readonly(self) -> Result<ReadonlyFile> {
        ReadonlyFile::new(
            self.path.clone(),
            ReadonlyFileEntity::from(match self.entity {
                WriteableFileEntity::TextFile(file) => ReadonlyFileEntity::TextFile(Arc::new(file)),
                WriteableFileEntity::Directory(dir) => ReadonlyFileEntity::Directory(Arc::new(dir)),
            }),
        )
        .context("Failed to create readonly file")
    }
}
