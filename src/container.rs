pub mod container;
use crate::custom_rc::FileLink;
use crate::custom_rc::SymlinkLink;
use crate::remote_exec::ExecutionOutput;
use anyhow::Result;
use std::collections::HashMap;
pub use std::path::PathBuf;
use tokio::sync::MutexGuard;

pub trait Container {
    async fn execute<'a, FileLinkType: FileLink, SymlinkLinkType: SymlinkLink<'a, FileLinkType>>(
        &self,
        cmd: &str,
        envs: HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        file_links: HashMap<PathBuf, MutexGuard<'a, FileLinkType>>,
    ) -> Result<ExecutionOutput>;
}
