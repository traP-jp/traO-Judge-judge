use std::path::PathBuf;
use anyhow::Context;

use crate::remote_exec::RemoteExecutor as RemoteExecutorTrait;
use super::Container as ContainerTrait;
use crate::custom_rc::FileLink as FileLinkTrait;

struct Container<RemoteExecutorType: RemoteExecutorTrait> {
    pub symlink_base_path: PathBuf,
    pub remote_executor: RemoteExecutorType,
}

impl<RemoteExecutorType: RemoteExecutorTrait> Container<RemoteExecutorType> {
    pub fn new(symlink_base_path: PathBuf, remote_executor: RemoteExecutorType) -> Self {
        Self {
            symlink_base_path,
            remote_executor,
        }
    }
}

impl<RemoteExecutorType: RemoteExecutorTrait> ContainerTrait for Container<RemoteExecutorType> {
    fn execute<
        'a,
        FileLinkType: FileLinkTrait<'a>,
    >(
        &self,
        cmd : &str,
        envs: std::collections::HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        file_links: &'a std::collections::HashMap<PathBuf, FileLinkType>,
    ) -> anyhow::Result<crate::remote_exec::ExecutionOutput> {
        let _symlinks = file_links
            .iter()
            .map(|(path, file_link)| {
                let symlink_path = self.symlink_base_path.join(path);
                file_link.symlink_to(&symlink_path)
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .context("Failed to create symlinks")?;
        self.remote_executor.execute(
            cmd,
            envs,
            connection_time_limit,
            execution_time_limit,
        )
            .context("Failed to execute command")
    }
}
