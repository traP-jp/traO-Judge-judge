mod dir_entity_factory;
mod file_entity;
pub mod file_link;
pub mod file_link_factory;
mod text_entity_factory;
use crate::text_resource_repository::TextResourceRepository as RepoTrait;
use anyhow::Result;
use std::path::PathBuf;
use tokio::sync::MutexGuard;

pub trait FileLink: Sized {}

pub trait SymlinkLink<'a, FileLink>: Sized {
    async fn new(target: MutexGuard<'a, FileLink>, destination: PathBuf) -> Result<Self>;
}

pub trait FileLinkFactory<
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    RepoType: RepoTrait<ExternalAccessKey>,
    FileLinkType: FileLink,
>
{
    async fn get_text_file_link(
        &self,
        text_resource_id: ExternalAccessKey,
    ) -> Result<FileLinkType>;
    async fn get_text_file_links(
        &self,
        text_resource_id: ExternalAccessKey,
        count: usize,
    ) -> Result<Vec<FileLinkType>>;
    async fn get_directory_link(&self) -> Result<FileLinkType>;
}
