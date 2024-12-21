use crate::remote_exec::ExecutionOutput;
use anyhow::Result;
use std::collections::HashMap;
use crate::custom_rc::{ReadonlyFile, WriteableFile, FileLink};
use crate::spmc_oneshot::SpmcReceiver;
use std::path::PathBuf;
use uuid::Uuid;

pub trait Container {
    async fn execute<
        'a,
        ReadonlyFileType: ReadonlyFile,
        WriteableFileType: WriteableFile<ReadonlyFileType>,
        ReadonlyFileLinkType: FileLink<'a, ReadonlyFileType>,
        WriteableFileLinkType: FileLink<'a, WriteableFileType>,
    > (
        &self,
        cmd: &str,
        envs: HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        readonly_files: HashMap<Uuid, (PathBuf, ReadonlyFileType)>,
        writeable_files: HashMap<Uuid, (PathBuf, WriteableFileType)>,
    ) -> Result<(
        ExecutionOutput,
        HashMap<Uuid, ReadonlyFileType>,
    )>;

    fn resource_destination_path(&self) -> PathBuf;
}

pub trait ContainerFactory<ContainerType: Container, Priority: Ord> {
    // 任意の2つのrxの組について、priorityによる順序とget_rxの呼び出し順が同じなら、その順に割り当てられる必要がある。
    async fn get_rx(&self, priority: Priority) -> Result<SpmcReceiver<ContainerType>>;
}
