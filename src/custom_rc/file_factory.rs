use super::{
    entity::{
        file_entity::*,
        dir_entity_factory::*,
        text_entity_factory::*,
    },
    readonly_file::*,
    writeable_file::*,
};
use crate::text_resource_repository::TextResourceRepository as RepoTrait;
use anyhow::{Context, Result};
use byte_unit::Byte;
use std::path::PathBuf;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct FileFactory<
    RepoType: RepoTrait,
>
 {
    directory_factory: DirEntityFactory,
    text_factory: Mutex<TextEntityFactory<RepoType>>,
    _phantom: std::marker::PhantomData<(
        WriteableFile,
        ReadonlyFile,
    )>,
}


impl<
    RepoType: RepoTrait,
> FileFactory<RepoType>
{
    pub fn new(
        base_path: PathBuf,
        repo: RepoType,
        text_factory_limit: Byte,
    ) -> Self {
        let dir_factory_path = base_path.clone().join("dir_factory");
        let text_factory_path = base_path.clone().join("text_factory");
        Self {
            directory_factory: DirEntityFactory::new(dir_factory_path),
            text_factory: Mutex::new(TextEntityFactory::new(
                text_factory_path,
                text_factory_limit,
                repo,
                0.1,
            )),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<
    RepoType: RepoTrait,
> super::FileFactory<
    WriteableFile,
    ReadonlyFile,
> for FileFactory<
    RepoType,
> {
    async fn new_textfile(&self, path: PathBuf, key: Uuid) -> Result<ReadonlyFile> {
        let file = {
            let text_entity_factory = self.text_factory.lock().await;
            text_entity_factory
                .get_text_file_entity(key, true)
                .await?
        };
        ReadonlyFile::new(path, ReadonlyFileEntity::TextFile(file))
    }

    async fn new_directory(&self, path: PathBuf) -> Result<WriteableFile> {
        let dir = self.directory_factory
            .get_dir_entity()
            .await
            .context("Failed to get directory entity")?;
        WriteableFile::new(path, WriteableFileEntity::Directory(dir))
    }
}
