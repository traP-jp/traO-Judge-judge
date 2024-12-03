use crate::custom_rc::file_entity::*;
use std::sync::Arc;

enum FileEntity {
    TextFileEntity(Arc<TextFileEntity>),
    DirectoryEntity(DirectoryEntity),
    SymlinkEntity(SymlinkEntity, Box<FileLink>),
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

    pub fn new_directory_link(directory_entity: DirectoryEntity) -> Self {
        Self {
            file_entity: FileEntity::DirectoryEntity(directory_entity),
        }
    }
}

impl crate::custom_rc::FileLink for FileLink {
    fn symlink_to(self, path: &std::path::PathBuf) -> anyhow::Result<Self>
    {
        let target_path = match &self.file_entity {
            FileEntity::TextFileEntity(text_file_entity) => text_file_entity.path.clone(),
            FileEntity::DirectoryEntity(directory_entity) => directory_entity.path.clone(),
            FileEntity::SymlinkEntity(symlink_entity, _) => symlink_entity.path.clone(),
        };
        let symlink_entity = SymlinkEntity::new(path.clone(), &target_path)?;
        let file_entity: FileEntity = FileEntity::SymlinkEntity(symlink_entity, Box::new(self));
        Ok(FileLink { file_entity })
    }
}
