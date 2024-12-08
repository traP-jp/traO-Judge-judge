use crate::text_resource_repository::traits::TextResourceRepository as RepoTrait;
use anyhow::Result;
use std::path::PathBuf;

pub trait FileLink<'a>: Sized {
    fn symlink_to(&'a self, path: &PathBuf) -> Result<Self>;
}

pub trait FileLinkFactory<
    'a,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    RepoType: RepoTrait<ExternalAccessKey>,
    FileLinkType: FileLink<'a>,
>
{
    fn get_text_file_link(&mut self, text_resource_id: ExternalAccessKey) -> Result<FileLinkType>;
    fn get_directory_link(&self) -> Result<FileLinkType>;
}
