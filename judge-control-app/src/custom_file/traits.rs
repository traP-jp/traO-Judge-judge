#![allow(drop_bounds)]
use anyhow::Result;
use std::path::PathBuf;
use uuid::Uuid;

pub trait File: Sized + Drop {
    type InitArgs;
    fn new(path: PathBuf, args: Self::InitArgs) -> Result<Self>;
    fn create_hardlink_to(&self, path: PathBuf) -> Result<Self>;
}

pub trait FileFactory: Sized {
    fn new(path: PathBuf) -> Result<Self>;
    fn create_file<FileType: File>(&self, uuid: Uuid, args: FileType::InitArgs)
        -> Result<FileType>;
    fn create_hardlink_of<FileType: File>(
        &self,
        uuid: Uuid,
        original: &FileType,
    ) -> Result<FileType>;
}
