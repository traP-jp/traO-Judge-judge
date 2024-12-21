use std::collections::HashMap;
use anyhow::{Result, Context};
use crate::custom_rc::{ReadonlyFile, WriteableFile, FileLink};
use crate::remote_exec::ExecutionOutput;
use uuid::Uuid;

pub async fn single_run<
    'a,
    ContainerType: crate::container::Container,
    WriteableFileType: WriteableFile<ReadonlyFileType>,
    ReadonlyFileType: ReadonlyFile,
    ReadonlyFileLinkType: FileLink<'a, ReadonlyFileType>,
    WriteableFileLinkType: FileLink<'a, WriteableFileType>,
> (
    cmd: &str,
    envs: HashMap<String, String>,
    connection_time_limit: std::time::Duration,
    execution_time_limit: std::time::Duration,
    container_rx: crate::spmc_oneshot::SpmcReceiver<ContainerType>,
    filename_dict: HashMap<Uuid, String>,
    writeable_files: HashMap<Uuid, WriteableFileType>,
    readonly_files: HashMap<Uuid, ReadonlyFileType>,
) -> Result<(ExecutionOutput, HashMap<Uuid, ReadonlyFileType>)> {
    // acquire container
    let container = container_rx
        .await
        .context("Failed to receive container")?;
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
    let writeable_file_refs = writeable_files
        .into_iter()
        .map(|(uuid, writeable_file)| {
            let filename = filename_dict
                .get(&uuid)
                .with_context(|| format!("Failed to get filename for {:?}", uuid))?;
            let path = destination_path.join(filename);
            Ok((uuid, (path, writeable_file)))
        })
        .collect::<Result<HashMap<Uuid, (std::path::PathBuf, WriteableFileType)>>>()?;

    // execute
    let (output, readonly_files) = container
        .execute::<
            ReadonlyFileType,
            WriteableFileType,
            ReadonlyFileLinkType,
            WriteableFileLinkType,
        > (
            cmd,
            envs,
            connection_time_limit,
            execution_time_limit,
            readonly_file_refs,
            writeable_file_refs,
        )
        .await
        .context("Failed to execute")?;

    Ok((output, readonly_files))
}
