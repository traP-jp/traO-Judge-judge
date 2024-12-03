use crate::custom_rc::file_entity::*;
use std::sync::Arc;

use super::file_entity;

enum FileEntity <'a> {
    TextFileEntity(Arc<TextFileEntity>),
    DirectoryEntity(DirectoryEntity),
    SymlinkEntity(SymlinkEntity, &'a FileEntity<'a>),
}

pub struct FileLink <'a> {
    file_entity: FileEntity<'a>,
}

impl<'a> FileLink<'a> {
    pub fn new_text_file_link(text_file_entity: Arc<TextFileEntity>) -> Self {
        Self {
            file_entity: FileEntity::TextFileEntity(text_file_entity),
        }
    }

    pub fn new_directory_link(directory_entity: DirectoryEntity) -> Self {
        Self {
            file_entity: FileEntity::DirectoryEntity(directory_entity),
        }
    }
}

impl<'a> crate::custom_rc::traits::FileLink<'a> for FileLink<'a> {
    fn symlink_to(&'a self, path: &std::path::PathBuf) -> anyhow::Result<Self>
    {
        let target_path = match &self.file_entity {
            FileEntity::TextFileEntity(text_file_entity) => text_file_entity.path.clone(),
            FileEntity::DirectoryEntity(directory_entity) => directory_entity.path.clone(),
            FileEntity::SymlinkEntity(symlink_entity, _) => symlink_entity.path.clone(),
        };
        let symlink_entity = SymlinkEntity::new(path.clone(), &target_path)?;
        let file_entity: FileEntity<'a> = FileEntity::SymlinkEntity(symlink_entity, &self.file_entity);
        Ok(FileLink { file_entity })
    }
}
