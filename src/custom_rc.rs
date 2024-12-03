mod dir_entity_factory;
mod file_entity;
pub mod file_link;
pub mod file_link_factory;
mod text_entity_factory;
use crate::text_resource_repository::TextResourceRepository as RepoTrait;
use anyhow::Result;
use std::path::PathBuf;

pub trait FileLink: Sized {
    fn symlink_to(self, path: &PathBuf) -> Result<Self>;
}

pub trait FileLinkFactory<
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    RepoType: RepoTrait<ExternalAccessKey>,
    FileLinkType: FileLink,
>
{
    fn get_text_file_link(&mut self, text_resource_id: ExternalAccessKey) -> Result<FileLinkType>;
    fn get_directory_link(&self) -> Result<FileLinkType>;
}
