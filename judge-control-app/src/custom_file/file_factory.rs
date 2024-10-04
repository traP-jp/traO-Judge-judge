use std::path::PathBuf;

use anyhow::{anyhow, Result};
use uuid::Uuid;

use super::traits::{self, File};

pub struct FileFactory {
    path: PathBuf,
}

impl traits::FileFactory for FileFactory {
    // ベースとなる path を指定
    fn new(path: PathBuf) -> Result<Self> {
        if path.is_dir() {
            Ok(FileFactory { path })
        } else {
            Err(anyhow!("path must be an existing dir"))
        }
    }
    // path/{uuid} にファイルまたはディレクトリを作成
    fn create_file<FileType: File>(
        &self,
        uuid: Uuid,
        args: FileType::InitArgs,
    ) -> Result<FileType> {
        let path = self.path.join(uuid.to_string());
        FileType::new(path, args)
    }
    // path/{uuid} に original のハードリンクを作成
    fn create_hardlink_of<FileType: File>(
        &self,
        uuid: Uuid,
        original: &FileType,
    ) -> Result<FileType> {
        let path = self.path.join(uuid.to_string());
        original.create_hardlink_to(path)
    }
}
