use std::{
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;

use super::traits::File;

pub struct TextFile {
    path: PathBuf,
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
    fn create_hardlink_to(&self, path: PathBuf) -> Result<Self> {
        std::fs::hard_link(&self.path, &path)?;
        Ok(TextFile { path })
    }
}
impl Drop for TextFile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("{:?}", e);
        }
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
