use std::path::PathBuf;

struct FileFactory {
    path: PathBuf,
}

impl super::traits::FileFactory for FileFactory {
    fn new(path: std::path::PathBuf) -> anyhow::Result<Self> {
        Ok(Self { path })
    }
    fn create_file<FileType: super::traits::File>(
        &self,
        uuid: uuid::Uuid,
        args: FileType::InitArgs,
    ) -> anyhow::Result<FileType> {
        let path = self.path.join(uuid.to_string());
        FileType::new(path, args)
    }
    fn create_symlink_of<FileType: super::traits::FileLink>(
        &self,
        uuid: uuid::Uuid,
        original: &FileType,
    ) -> anyhow::Result<FileType> {
        let path = self.path.join(uuid.to_string());
        original.create_symlink_to(path)
    }
}
