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

pub struct FileFactory<
    WriteableFileType: super::WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: super::ReadonlyFile,
    RepoType: RepoTrait<ExternalAccessKey>,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
>
 {
    directory_factory: DirEntityFactory,
    text_factory: TextEntityFactory<ExternalAccessKey, RepoType>,
    _phantom: std::marker::PhantomData<(
        WriteableFileType,
        ReadonlyFileType,
    )>,
}


impl<
    WriteableFileType: super::WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: super::ReadonlyFile,
    RepoType: RepoTrait<ExternalAccessKey>,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
> FileFactory<WriteableFileType, ReadonlyFileType, RepoType, ExternalAccessKey>
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
            text_factory: TextEntityFactory::new(
                text_factory_path,
                text_factory_limit,
                repo,
                0.1,
            ),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<
    WriteableFileType: super::WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: super::ReadonlyFile,
    RepoType: RepoTrait<ExternalAccessKey>,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
> super::FileFactory<
    WriteableFileType,
    ReadonlyFileType,
    ExternalAccessKey
> for FileFactory<
    WriteableFileType,
    ReadonlyFileType,
    RepoType,
    ExternalAccessKey
> {
    fn new_textfile(path: PathBuf, key: ExternalAccessKey) -> Result<WriteableFileType> {
        let file = TextFileEntity::new(path, key);
        Ok(WriteableFileType::new(file))
    }

    fn new_directory(path: PathBuf) -> Result<WriteableFileType> {
        let dir = DirEntityFactory::new(path);
        Ok(WriteableFileType::new(dir))
    }
}
