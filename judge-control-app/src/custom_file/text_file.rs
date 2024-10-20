#[cfg(test)]
mod tests;

use std::{
    io::Write,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use super::traits::{File, FileEntity, FileLink};

struct TextFileEntity {
    path: PathBuf,
}

impl FileEntity for TextFileEntity {}

impl File for TextFileEntity {
    type InitArgs = String;
    fn new(path: PathBuf, args: Self::InitArgs) -> anyhow::Result<Self> {
        let mut file = std::fs::File::create(&path)?;
        file.write_all(args.as_bytes())?;
        Ok(Self { path })
    }
}

impl Drop for TextFileEntity {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        unimplemented!("error handling for file deletion failure");
    }
}

struct TextFileLink {
    path: PathBuf,
    entity: Arc<RwLock<TextFileEntity>>,
}

impl FileLink for TextFileLink {
    fn create_symlink_to(&self, path: PathBuf) -> anyhow::Result<Self> {
        Self::new(path, self.entity.clone())
    }
}

impl File for TextFileLink {
    type InitArgs = Arc<RwLock<TextFileEntity>>;
    fn new(path: PathBuf, args: Self::InitArgs) -> anyhow::Result<Self> {
        std::os::unix::fs::symlink(&args.read().unwrap().path, &path)?;
        Ok(Self {
            path,
            entity: args.clone(),
        })
    }
}

impl Drop for TextFileLink {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        unimplemented!("error handling for file deletion failure");
    }
}
