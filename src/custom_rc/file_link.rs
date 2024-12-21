use std::path::PathBuf;
use anyhow::{Result, Context};

pub struct FileLink<FileType: super::File> {
    path: PathBuf,
    file: FileType,
}

impl<FileType: super::File> super::FileLink<FileType> for FileLink<FileType> {
    fn link(file: FileType, path: PathBuf) -> Result<Self> {
        let target = file.path();
        std::os::unix::fs::symlink(&target, &path)
            .with_context(|| format!("Failed to create symlink from {:?} to {:?}", target, path))?;
        Ok(Self { path, file })
    }

    fn unlink(self) -> Result<FileType> {
        std::fs::remove_file(&self.path)
            .with_context(|| format!("Failed to remove symlink {:?}", self.path))?;
        Ok(self.file)
    }
}
