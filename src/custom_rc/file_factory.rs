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

pub struct FileFactory<
    RepoType: RepoTrait<ExternalAccessKey>,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
>
 {
    directory_factory: DirEntityFactory,
    text_factory: Mutex<TextEntityFactory<ExternalAccessKey, RepoType>>,
    _phantom: std::marker::PhantomData<(
        WriteableFile,
        ReadonlyFile,
    )>,
}


impl<
    RepoType: RepoTrait<ExternalAccessKey>,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
> FileFactory<RepoType, ExternalAccessKey>
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
    RepoType: RepoTrait<ExternalAccessKey>,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
> super::FileFactory<
    WriteableFile,
    ReadonlyFile,
    ExternalAccessKey
> for FileFactory<
    RepoType,
    ExternalAccessKey
> {
    async fn new_textfile(&self, path: PathBuf, key: ExternalAccessKey) -> Result<ReadonlyFile> {
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
