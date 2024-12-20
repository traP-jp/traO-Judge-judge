mod entity;
pub mod readonly_file;
pub mod writeable_file;
pub mod file_factory;
use anyhow::Result;
use std::path::PathBuf;


pub trait WriteableFile<ReadonlyFileType: ReadonlyFile> {
    fn to_readonly(&self) -> ReadonlyFileType;
}

pub trait ReadonlyFile: Clone {}

pub trait FileFactory<
    WriteableFileType: WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: ReadonlyFile,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
> {
    fn new_textfile(path: PathBuf, key: ExternalAccessKey) -> Result<WriteableFileType>;
    fn new_directory(path: PathBuf) -> Result<WriteableFileType>;
}

pub enum File<
    WriteableFileType: WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: ReadonlyFile,
> {
    Writeable(WriteableFileType),
    Readonly(ReadonlyFileType),
}
