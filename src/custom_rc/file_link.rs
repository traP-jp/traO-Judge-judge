use crate::custom_rc::file_entity::{TextFileEntity, DirectoryEntity};
use std::sync::Arc;

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

    pub fn new_directory_link(directory_entity: DirectoryEntity) -> Self {
        Self {
            file_entity: FileEntity::DirectoryEntity(directory_entity),
        }
    }
}

impl crate::custom_rc::traits::FileLink for FileLink {
    fn symlink_to(&self, target: &std::path::PathBuf) -> anyhow::Result<()> {
        match &self.file_entity {
            FileEntity::TextFileEntity(text_file_entity) => {
                std::os::unix::fs::symlink(&text_file_entity.path, target)
                    .map_err(|e| anyhow::anyhow!("Failed to create symlink : {:?}", e))
            }
            FileEntity::DirectoryEntity(directory_entity) => {
                std::os::unix::fs::symlink(&directory_entity.path, target)
                    .map_err(|e| anyhow::anyhow!("Failed to create symlink : {:?}", e))
            }
        }
    }
}
