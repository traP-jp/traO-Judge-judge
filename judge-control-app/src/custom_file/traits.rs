#![allow(drop_bounds)]
use std::path::PathBuf;
use anyhow::Result;
use uuid::Uuid;

pub trait File: Sized + Drop {
    type InitArgs;
    fn new(path: PathBuf, args: Self::InitArgs) -> Result<Self>;
    fn get_path(&self) -> PathBuf;
}

pub trait FileEntity: File {}

pub trait FileLink: File {}

pub trait FileFactory: Sized {
    fn new(path: PathBuf) -> Result<Self>;
    fn get_file_entity<FileType: File>(&self, uuid: Uuid, args: FileType::InitArgs) -> Result<FileType>;
}
