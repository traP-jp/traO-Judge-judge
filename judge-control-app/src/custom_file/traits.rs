#![allow(drop_bounds)]
use anyhow::{anyhow, Result};
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
        std::fs::create_dir_all(&path)?;
        Ok(Directory { path })
    }
    fn get_hardlink_to(&self, path: PathBuf) -> Result<Self> {
        Err(anyhow!("hard link not allowed for directory"))
    }
}

impl File for TextFile {
    type InitArgs = todo!();
    fn new(path: PathBuf, args: Self::InitArgs) -> Result<Self> {
        std::fs::File::create(&path)?;
        Ok(TextFile { path })
    }
    fn get_hardlink_to(&self, path: PathBuf) -> Result<Self> {
        std::fs::hard_link(&self.path, &path)?;
        TextFile::new(path, todo!())
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path);
    }
}

impl Drop for TextFile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path);
    }
}

impl TextFile {
    fn read(&self) -> Result<String> {
        let mut f = OpenOptions::new().read(true).open(&self.path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(contents)
    }
    fn write(&self, contents: String) -> Result<()> {
        let mut f = OpenOptions::new().write(true).open(&self.path)?;
        f.write_all(contents.as_bytes())?;
        Ok(())
    }
}
