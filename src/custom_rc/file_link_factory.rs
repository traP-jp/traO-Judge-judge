use crate::custom_rc::dir_entity_factory::DirEntityFactory;
use crate::custom_rc::file_link::FileLink;
use crate::custom_rc::text_entity_factory::TextEntityFactory;
use crate::text_resource_repository::traits::TextResourceRepository as RepoTrait;

pub struct FileLinkFactory<
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    RepoType: RepoTrait<ExternalAccessKey>,
> {
    text_entity_factory: TextEntityFactory<ExternalAccessKey, RepoType>,
    dir_entity_factory: DirEntityFactory,
}

impl<
        ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
        RepoType: RepoTrait<ExternalAccessKey>,
    > FileLinkFactory<ExternalAccessKey, RepoType>
{
    pub fn new(
        text_entity_factory_path: std::path::PathBuf,
        dir_entity_factory_path: std::path::PathBuf,
        text_entity_factory_cache_dir_size_limit: byte_unit::Byte,
        text_entity_cache_ratio: f64,
        external_repo: RepoType,
    ) -> Self {
        Self {
            text_entity_factory: TextEntityFactory::new(
                text_entity_factory_path,
                text_entity_factory_cache_dir_size_limit,
                external_repo,
                text_entity_cache_ratio,
            ),
            dir_entity_factory: DirEntityFactory::new(dir_entity_factory_path),
        }
    }
}

impl<
        'a,
        ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
        RepoType: RepoTrait<ExternalAccessKey>,
    > crate::custom_rc::traits::FileLinkFactory<'a, ExternalAccessKey, RepoType, FileLink<'a>>
    for FileLinkFactory<ExternalAccessKey, RepoType>
{
    fn get_text_file_link(
        &mut self,
        text_resource_id: ExternalAccessKey,
    ) -> anyhow::Result<FileLink<'a>> {
        let text_file_entity = self
            .text_entity_factory
            .get_text_file_entity(text_resource_id)?;
        Ok(FileLink::new_text_file_link(text_file_entity))
    }

    fn get_directory_link(&self) -> anyhow::Result<FileLink<'a>> {
        let directory_entity = self.dir_entity_factory.get_dir_entity()?;
        Ok(FileLink::new_directory_link(directory_entity))
    }
}
