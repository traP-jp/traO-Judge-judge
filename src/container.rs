use crate::remote_exec::ExecutionOutput;
use anyhow::Result;
use std::collections::HashMap;
use crate::custom_rc::{ReadonlyFile, WriteableFile, FileLink};
use crate::spmc_oneshot::SpmcReceiver;

pub trait Container {
    async fn execute<
        ReadonlyFileType: ReadonlyFile,
        WriteableFileType: WriteableFile<ReadonlyFileType>,
        ReadonlyFileLinkType: FileLink<ReadonlyFileType>,
        WriteableFileLinkType: FileLink<WriteableFileType>,
    > (
        &self,
        cmd: &str,
        envs: HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        readonly_files: &Vec<ReadonlyFileType>,
        writeable_files: &Vec<WriteableFileType>,
    ) -> Result<ExecutionOutput>;
}

pub trait ContainerFactory<ContainerType: Container, Priority: Ord> {
    async fn get_resv(&self, priority: Priority) -> Result<SpmcReceiver<ContainerType>>;
}
