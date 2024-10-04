use std::path::PathBuf;

use anyhow::{anyhow, Result};

use super::traits::File;

pub struct Directory {
    path: PathBuf,
}

impl File for Directory {
    type InitArgs = ();
    fn new(path: PathBuf, _args: Self::InitArgs) -> Result<Self> {
        std::fs::create_dir_all(&path)?;
        Ok(Directory { path })
    }
    fn create_hardlink_to(&self, _path: PathBuf) -> Result<Self> {
        Err(anyhow!("hard link not allowed for directory"))
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_dir_all(&self.path) {
            eprintln!("{:?}", e);
        }
    }
}
