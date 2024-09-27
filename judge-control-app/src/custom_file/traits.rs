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
        original: FileType,
    ) -> Result<FileType>;
}

pub enum CustomFile {
    Directory(Directory),
    TextFile(TextFile),
}

pub struct Directory {
    path: PathBuf,
}

pub struct TextFile {
    path: PathBuf,
}

impl File for Directory {
    type InitArgs = todo!();
    fn new(path: PathBuf, args: Self::InitArgs) -> Result<Self> {
        todo!();
    }
    fn get_hardlink_to(&self, path: PathBuf) -> Result<Self> {
        todo!();
    }
}

impl File for TextFile {
    type InitArgs = todo!();
    fn new(path: PathBuf, args: Self::InitArgs) -> Result<Self> {
        todo!();
    }
    fn get_hardlink_to(&self, path: PathBuf) -> Result<Self> {
        todo!();
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        todo!();
    }
}

impl Drop for TextFile {
    fn drop(&mut self) {
        todo!();
    }
}

impl TextFile {
    fn read(&self) -> String {
        todo!();
    }
    fn write(&self, contents: String) {
        todo!();
    }
}
