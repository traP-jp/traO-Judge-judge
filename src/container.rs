pub mod container;
use std::path::PathBuf;
use crate::custom_rc::FileLink;
use std::collections::HashMap;
use anyhow::Result;
use crate::remote_exec::ExecutionOutput;

pub trait Container {
    fn execute<
        'a,
        FileLinkType: FileLink<'a>,
    >(
        &self,
        cmd : &str,
        envs: HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration,
        file_links: &'a HashMap<PathBuf, FileLinkType>,
    ) -> Result<ExecutionOutput>;
}
