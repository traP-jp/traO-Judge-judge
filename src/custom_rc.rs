mod entity;
pub mod readonly_file;
pub mod writeable_file;
pub mod file_factory;
pub mod file_link;
use anyhow::Result;
use std::path::PathBuf;
use uuid::Uuid;

pub trait File {
    fn path(&self) -> PathBuf;
}

pub trait WriteableFile<ReadonlyFileType: ReadonlyFile>: File {
    async fn to_readonly(self) -> Result<ReadonlyFileType>;
}

pub trait ReadonlyFile: Clone + File {}

pub trait FileLink<FileType: File>: Sized {
    fn link(file: FileType, path: PathBuf) -> Result<Self>;
    fn unlink(self) -> Result<FileType>;
}

pub trait FileFactory<
    WriteableFileType: WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: ReadonlyFile,
> {
    async fn new_textfile(&self, key: &Uuid) -> Result<ReadonlyFileType>;
    async fn new_textfile_from_raw(&self, raw: &str) -> Result<WriteableFileType>;
    async fn new_directory(&self) -> Result<WriteableFileType>;
}

pub enum FileEnum<
    WriteableFileType: WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: ReadonlyFile,
> {
    Writeable(WriteableFileType),
    Readonly(ReadonlyFileType),
}
