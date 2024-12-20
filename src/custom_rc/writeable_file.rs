use super::entity::file_entity::*;
use super::readonly_file::ReadonlyFile;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Result, Context};


pub struct WriteableFile {
    pub path: PathBuf,
    pub entity: FileEntity,
}

impl WriteableFile {
    pub fn new(path: PathBuf, entity: FileEntity) -> Result<Self> {
        let target_path = match entity {
            FileEntity::TextFile(_) => path,
            FileEntity::Directory(_) => path,
        };
        std::os::unix::fs::symlink(&target_path, &path)
            .with_context(|| format!("Failed to create symlink from {:?} to {:?}", target_path, path))?;
        Ok(Self { path, entity })
    }
}

impl super::WriteableFile<ReadonlyFile> for WriteableFile {
    fn to_readonly(&self) -> ReadonlyFile {
        ReadonlyFile::new(self.path.clone(), Arc::new(self.entity.clone()))
    }
}