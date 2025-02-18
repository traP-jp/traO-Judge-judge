use crate::custom_rc::{FileLink, ReadonlyFile, WritableFile};
use crate::remote_exec::ExecutionOutput;
use crate::spmc_oneshot::SpmcReceiver;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

pub trait Container {
    async fn execute<
        ReadonlyFileType: ReadonlyFile,
        WritableFileType: WritableFile<ReadonlyFileType>,
        ReadonlyFileLinkType: FileLink<ReadonlyFileType>,
        WritableFileLinkType: FileLink<WritableFileType>,
    >(
        &self,
        cmd: &str,
        envs: HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        readonly_files: HashMap<Uuid, (PathBuf, ReadonlyFileType)>,
        writable_files: HashMap<Uuid, (PathBuf, WritableFileType)>,
    ) -> Result<(ExecutionOutput, HashMap<Uuid, ReadonlyFileType>)>;

    fn resource_destination_path(&self) -> PathBuf;
}

pub trait ContainerFactory<ContainerType: Container, Priority: Ord> {
    // 任意の2つのrxの組について、priorityによる順序とget_rxの呼び出し順が同じなら、その順に割り当てられる必要がある。
    async fn get_rx(&self, priority: Priority) -> Result<SpmcReceiver<ContainerType>>;
}
