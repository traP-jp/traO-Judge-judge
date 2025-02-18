use crate::custom_rc::{FileLink, ReadonlyFile, WritableFile};
use crate::remote_exec::ExecutionOutput;
use anyhow::{Context, Result};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn single_run<
    ContainerType: crate::container::Container,
    ReadonlyFileType: ReadonlyFile,
    WritableFileType: WritableFile<ReadonlyFileType>,
    ReadonlyFileLinkType: FileLink<ReadonlyFileType>,
    WritableFileLinkType: FileLink<WritableFileType>,
>(
    cmd: &str,
    envs: HashMap<String, String>,
    connection_time_limit: std::time::Duration,
    execution_time_limit: std::time::Duration,
    container_rx: crate::spmc_oneshot::SpmcReceiver<ContainerType>,
    filename_dict: HashMap<Uuid, String>,
    writable_files: HashMap<Uuid, WritableFileType>,
    readonly_files: HashMap<Uuid, ReadonlyFileType>,
) -> Result<(ExecutionOutput, HashMap<Uuid, ReadonlyFileType>)> {
    // acquire container
    let container = container_rx.await.context("Failed to receive container")?;
    let destination_path = container.resource_destination_path();

    // prepare readonly files
    let readonly_file_refs = readonly_files
        .into_iter()
        .map(|(uuid, readonly_file)| {
            let filename = filename_dict
                .get(&uuid)
                .with_context(|| format!("Failed to get filename for {:?}", uuid))?;
            let path = destination_path.join(filename);
            Ok((uuid, (path, readonly_file)))
        })
        .collect::<Result<HashMap<Uuid, (std::path::PathBuf, ReadonlyFileType)>>>()?;
    let writable_file_refs = writable_files
        .into_iter()
        .map(|(uuid, writable_file)| {
            let filename = filename_dict
                .get(&uuid)
                .with_context(|| format!("Failed to get filename for {:?}", uuid))?;
            let path = destination_path.join(filename);
            Ok((uuid, (path, writable_file)))
        })
        .collect::<Result<HashMap<Uuid, (std::path::PathBuf, WritableFileType)>>>()?;

    // execute
    let (output, readonly_files) = container
        .execute::<ReadonlyFileType, WritableFileType, ReadonlyFileLinkType, WritableFileLinkType>(
            cmd,
            envs,
            connection_time_limit,
            execution_time_limit,
            readonly_file_refs,
            writable_file_refs,
        )
        .await
        .context("Failed to execute")?;

    Ok((output, readonly_files))
}
