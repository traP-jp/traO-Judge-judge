use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::MutexGuard;

pub struct SingleExecutionArgs<'a, FileLinkType, JobOrderingType: Clone + Ord> {
    pub cmd: String,
    pub envs: HashMap<String, String>,
    pub file_links: HashMap<PathBuf, MutexGuard<'a, FileLinkType>>,
    pub connection_time_limit: Duration,
    pub execution_time_limit: Duration,
    pub job_order: JobOrderingType,
}