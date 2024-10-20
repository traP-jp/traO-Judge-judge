use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use super::traits::{File, FileEntity, FileLink};

struct DirectoryEntity {
    path: PathBuf,
}

impl FileEntity for DirectoryEntity {}

impl File for DirectoryEntity {
    type InitArgs = ();
    fn new(path: PathBuf, _args: Self::InitArgs) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Drop for DirectoryEntity {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
        unimplemented!("error handling for file deletion failure");
    }
}

struct DirectoryLink {
    path: PathBuf,
    entity: Arc<RwLock<DirectoryEntity>>,
}

impl FileLink for DirectoryLink {
    fn create_symlink_to(&self, path: PathBuf) -> anyhow::Result<Self> {
        Self::new(path, self.entity.clone())
    }
}

impl File for DirectoryLink {
    type InitArgs = Arc<RwLock<DirectoryEntity>>;
    fn new(path: PathBuf, args: Self::InitArgs) -> anyhow::Result<Self> {
        std::os::unix::fs::symlink(&args.read().unwrap().path, &path)?;
        Ok(Self {
            path,
            entity: args.clone(),
        })
    }
}

impl Drop for DirectoryLink {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
        unimplemented!("error handling for file deletion failure");
    }
}
