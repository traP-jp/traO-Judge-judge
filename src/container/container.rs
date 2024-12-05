use anyhow::Context;
use std::path::PathBuf;

use super::Container as ContainerTrait;
use crate::custom_rc::FileLink as FileLinkTrait;
use crate::{
    custom_rc::SymlinkLink as SymlinkLinkTrait, remote_exec::RemoteExecutor as RemoteExecutorTrait,
};
use tokio::sync::MutexGuard;

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
    async fn execute<
        'a,
        FileLinkType: FileLinkTrait,
        SymlinkLinkType: SymlinkLinkTrait<'a, FileLinkType>,
    >(
        &self,
        cmd: &str,
        envs: std::collections::HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        file_links: std::collections::HashMap<PathBuf, MutexGuard<'a, FileLinkType>>,
    ) -> anyhow::Result<crate::remote_exec::ExecutionOutput> {
        let mut _symlinks = Vec::new();
        for (destination, target) in file_links {
            let symlink = SymlinkLinkType::new(target, destination)
                .await
                .context("Failed to create symlink")?;
            _symlinks.push(symlink);
        }
        self.remote_executor
            .execute(cmd, envs, connection_time_limit, execution_time_limit)
            .await
            .context("Failed to execute command")
    }
}
