#[cfg(test)]
mod tests;

use std::{
    io::Write,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use super::traits::File;

struct TextFileEntity {
    path: PathBuf,
}

impl File for TextFileEntity {
    type InitArgs = String;
    fn new(path: PathBuf, args: Self::InitArgs) -> anyhow::Result<Self> {
        let mut file = std::fs::File::create(&path)?;
        file.write_all(args.as_bytes())?;
        Ok(Self { path })
    }
    fn create_symlink_to(&self, path: PathBuf) -> anyhow::Result<Self> {
        unimplemented!();
    }
}

impl Drop for TextFileEntity {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path);
        eprintln!("Entity {:?} dropped.", self.path);
    }
}

struct TextFileLink {
    path: PathBuf,
    entity: Arc<RwLock<TextFileEntity>>,
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
    fn create_symlink_to(&self, path: PathBuf) -> anyhow::Result<Self> {
        Self::new(path, self.entity.clone())
    }
}

impl Drop for TextFileLink {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path);
        eprintln!("Link {:?} dropped.", self.path);
    }
}
