use crate::custom_rc::file_entity::*;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::MutexGuard;

enum FileEntity {
    TextFileEntity(Arc<TextFileEntity>),
    DirectoryEntity(DirectoryEntity),
}

pub struct FileLink {
    file_entity: FileEntity,
}

impl FileLink {
    pub fn new_text_file_link(text_file_entity: Arc<TextFileEntity>) -> Self {
        Self {
            file_entity: FileEntity::TextFileEntity(text_file_entity),
        }
    }

    pub fn new_text_file_links(text_file_entity: Arc<TextFileEntity>, count: usize) -> Vec<Self> {
        (0..count)
            .map(|_| Self::new_text_file_link(text_file_entity.clone()))
            .collect()
    }

    pub fn new_directory_link(directory_entity: DirectoryEntity) -> Self {
        Self {
            file_entity: FileEntity::DirectoryEntity(directory_entity),
        }
    }

    fn get_path(&self, destination: &PathBuf) -> anyhow::Result<PathBuf> {
        match &self.file_entity {
            FileEntity::TextFileEntity(text_file_entity) => Ok(text_file_entity.path.clone()),
            FileEntity::DirectoryEntity(directory_entity) => {
                Ok(directory_entity.path.join(destination))
            }
        }
    }
}

impl crate::custom_rc::FileLink for FileLink {}

pub struct SymlinkLink<'a> {
    file_entity: SymlinkEntity,
    target: MutexGuard<'a, FileLink>,
}

impl<'a> super::SymlinkLink<'a, FileLink> for SymlinkLink<'a> {
    async fn new(target: MutexGuard<'a, FileLink>, destination: PathBuf) -> Result<Self> {
        let target_path = target
            .get_path(&destination)
            .context("Failed to get path from target")?;
        let symlink_entity = SymlinkEntity::new(target_path, &destination).await?;
        Ok(Self {
            file_entity: symlink_entity,
            target,
        })
    }
}
