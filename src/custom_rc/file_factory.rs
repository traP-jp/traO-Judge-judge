use super::{
    entity::{file_entity::*, text_entity_factory::*, writable_entity_factory::*},
    readonly_file::*,
    writable_file::*,
};
use crate::text_resource_repository::TextResourceRepository as RepoTrait;
use anyhow::{Context, Result};
use byte_unit::Byte;
use std::path::PathBuf;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct FileFactory<RepoType: RepoTrait> {
    writable_entity_factory: WritableEntityFactory,
    text_factory: Mutex<TextEntityFactory<RepoType>>,
    base_path: PathBuf,
    _phantom: std::marker::PhantomData<(WritableFile, ReadonlyFile)>,
}

impl<RepoType: RepoTrait> FileFactory<RepoType> {
    pub fn new(base_path: PathBuf, repo: RepoType, text_factory_limit: Byte) -> Self {
        let dir_factory_path = base_path.clone().join("dir_factory");
        let text_factory_path = base_path.clone().join("text_factory");
        Self {
            writable_entity_factory: WritableEntityFactory::new(dir_factory_path),
            text_factory: Mutex::new(TextEntityFactory::new(
                text_factory_path,
                text_factory_limit,
                repo,
                0.1,
            )),
            base_path: base_path.join("files"),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<RepoType: RepoTrait> super::FileFactory<WritableFile, ReadonlyFile> for FileFactory<RepoType> {
    async fn new_textfile(&self, key: &Uuid) -> Result<ReadonlyFile> {
        let file = {
            let text_entity_factory = self.text_factory.lock().await;
            text_entity_factory
                .get_text_file_entity(key.clone(), true)
                .await?
        };

        let path = self.base_path.join(Uuid::new_v4().to_string());
        ReadonlyFile::new(path, ReadonlyFileEntity::TextFile(file))
    }

    async fn new_directory(&self) -> Result<WritableFile> {
        let dir = self
            .writable_entity_factory
            .get_dir_entity()
            .await
            .context("Failed to get directory entity")?;
        let path = self.base_path.join(Uuid::new_v4().to_string());
        WritableFile::new(path, WritableFileEntity::Directory(dir))
    }

    async fn new_textfile_from_raw(&self, raw: &str) -> Result<WritableFile> {
        let file = self
            .writable_entity_factory
            .get_text_file_entity(raw)
            .await
            .context("Failed to get text file entity")?;
        let path = self.base_path.join(Uuid::new_v4().to_string());
        WritableFile::new(path, WritableFileEntity::TextFile(file))
    }
}
