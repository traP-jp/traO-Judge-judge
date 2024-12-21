mod entity;
pub mod readonly_file;
pub mod writeable_file;
pub mod file_factory;
pub mod file_link;
use anyhow::Result;
use std::path::PathBuf;

pub trait File {
    fn path(&self) -> PathBuf;
}

pub trait WriteableFile<ReadonlyFileType: ReadonlyFile>: File {
    async fn to_readonly(self) -> Result<ReadonlyFileType>;
}

pub trait ReadonlyFile: Clone + File {}

pub trait FileLink<'a, FileType: File>: Sized {
    fn link(file: &'a mut FileType, path: PathBuf) -> Result<Self>;
    fn unlink(self) -> Result<()>;
}

pub trait FileFactory<
    WriteableFileType: WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: ReadonlyFile,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
> {
    async fn new_textfile(&self, path: PathBuf, key: ExternalAccessKey) -> Result<ReadonlyFileType>;
    async fn new_directory(&self, path: PathBuf) -> Result<WriteableFileType>;
}

pub enum FileEnum<
    WriteableFileType: WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: ReadonlyFile,
> {
    Writeable(WriteableFileType),
    Readonly(ReadonlyFileType),
}
