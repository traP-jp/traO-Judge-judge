use anyhow::{Context, Result};
use std::path::PathBuf;
pub struct TextFileEntity {
    pub path: PathBuf,
}

impl TextFileEntity {
    pub async fn new(path: PathBuf, content: &str) -> Result<Self> {
        std::fs::write(&path, content)
            .with_context(|| {
                format!(
                    "Failed to write to file while creating TextFileEntity : {:?}",
                    path
                )
            })
            .map(|_| Self { path })
    }
}

impl Drop for TextFileEntity {
    fn drop(&mut self) {
        let result = std::fs::remove_file(&self.path).context(format!(
            "Failed to remove file while dropping TextFileEntity : {:?}",
            self.path
        ));
        match result {
            Ok(_) => {
                return;
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}

pub struct DirectoryEntity {
    pub path: PathBuf,
}

impl DirectoryEntity {
    pub async fn new(path: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&path)
            .with_context(|| {
                format!(
                    "Failed to create directory while creating DirectoryEntity : {:?}",
                    path
                )
            })
            .map(|_| Self { path })
    }
}

impl Drop for DirectoryEntity {
    fn drop(&mut self) {
        let result = std::fs::remove_dir_all(&self.path).context(format!(
            "Failed to remove directory while dropping DirectoryEntity : {:?}",
            self.path
        ));
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}

pub struct SymlinkEntity {
    pub path: PathBuf,
}

impl SymlinkEntity {
    pub async fn new(path: PathBuf, target: &PathBuf) -> Result<Self> {
        std::os::unix::fs::symlink(target, &path)
            .with_context(|| {
                format!(
                    "Failed to create symlink while creating SymlinkEntity : {:?}",
                    path
                )
            })
            .map(|_| Self { path })
    }
}

impl Drop for SymlinkEntity {
    fn drop(&mut self) {
        let result = std::fs::remove_file(&self.path).context(format!(
            "Failed to remove symlink while dropping SymlinkEntity : {:?}",
            self.path
        ));
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}
