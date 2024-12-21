use std::path::PathBuf;
use anyhow::{Result, Context};

pub struct FileLink<'a, FileType: super::File> {
    path: PathBuf,
    file: &'a mut FileType,
}

impl<'a, FileType: super::File> super::FileLink<'a, FileType> for FileLink<'a, FileType> {
    fn link(file: &'a mut FileType, path: PathBuf) -> Result<Self> {
        let target = file.path();
        std::os::unix::fs::symlink(&target, &path)
            .with_context(|| format!("Failed to create symlink from {:?} to {:?}", target, path))?;
        Ok(Self { path, file })
    }

    fn unlink(self) -> anyhow::Result<()> {
        std::fs::remove_file(&self.path)
            .with_context(|| format!("Failed to remove symlink {:?}", self.path))?;
        Ok(())
    }
}
