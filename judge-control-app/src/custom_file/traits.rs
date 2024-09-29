#![allow(drop_bounds)]
use anyhow::{anyhow, Result};
use std::{
    io::{Read, Write},
    path::PathBuf,
};
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
    type InitArgs = ();
    fn new(path: PathBuf, _args: Self::InitArgs) -> Result<Self> {
        std::fs::create_dir_all(&path)?;
        Ok(Directory { path })
    }
    fn get_hardlink_to(&self, _path: PathBuf) -> Result<Self> {
        Err(anyhow!("hard link not allowed for directory"))
    }
}

impl File for TextFile {
    type InitArgs = Option<String>;
    fn new(path: PathBuf, args: Self::InitArgs) -> Result<Self> {
        std::fs::File::create(&path)?;
        let res = TextFile { path };
        if let Some(contents) = args {
            res.write(contents)?;
        }
        Ok(res)
    }
    fn get_hardlink_to(&self, path: PathBuf) -> Result<Self> {
        std::fs::hard_link(&self.path, &path)?;
        TextFile::new(path, None)
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
        let mut file = std::fs::OpenOptions::new().read(true).open(&self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    fn write(&self, contents: String) -> Result<()> {
        let mut file = std::fs::OpenOptions::new().write(true).open(&self.path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}

pub struct FileManager {
    path: PathBuf,
}

impl FileFactory for FileManager {
    fn new(path: PathBuf) -> Result<Self> {
        if path.is_dir() {
            Ok(FileManager { path })
        } else {
            Err(anyhow!("path must be an existing dir"))
        }
    }
    fn get_file<FileType: File>(&self, uuid: Uuid, args: FileType::InitArgs) -> Result<FileType> {
        let path = self.path.join(uuid.to_string());
        FileType::new(path, args)
    }
    fn get_hardlink_of<FileType: File>(&self, uuid: Uuid, original: FileType) -> Result<FileType> {
        let path = self.path.join(uuid.to_string());
        original.get_hardlink_to(path)
    }
}
