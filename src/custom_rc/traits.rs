use std::path::PathBuf;
use anyhow::Result;
use crate::text_resource_repository::traits::TextResourceRepository as RepoTrait;

pub trait FileLink {
    fn symlink_to(&self, target: &PathBuf) -> Result<()>;
}

pub trait FileLinkFactory<
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    RepoType: RepoTrait<ExternalAccessKey>,
    FileLinkType: FileLink,
> {
    fn get_text_file_link(&mut self, text_resource_id: ExternalAccessKey) -> Result<FileLinkType>;
    fn get_directory_link(&self) -> Result<FileLinkType>;
}