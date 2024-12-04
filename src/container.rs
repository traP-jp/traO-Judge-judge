pub mod container;
use std::path::PathBuf;
use crate::custom_rc::FileLink;
use std::collections::HashMap;
use anyhow::Result;
use crate::remote_exec::ExecutionOutput;
use tokio::sync::MutexGuard;
use crate::custom_rc::SymlinkLink;

pub trait Container<'a> {
    async fn execute<
        FileLinkType: FileLink,
        SymlinkLinkType: SymlinkLink<'a, FileLinkType>,
    >(
        &self,
        cmd : &str,
        envs: HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        file_links: HashMap<PathBuf, MutexGuard<'a, FileLinkType>>,
    ) -> Result<ExecutionOutput>;
}
