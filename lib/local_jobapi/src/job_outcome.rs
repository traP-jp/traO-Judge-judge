use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct JobOutcome {
    path: Arc<JobOutcomeInner>,
}

#[derive(Debug)]
struct JobOutcomeInner {
    pub path: PathBuf,
}

impl JobOutcome {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: Arc::new(JobOutcomeInner { path }),
        }
    }

    pub(crate) fn path(&self) -> &PathBuf {
        &self.path.path
    }
}

impl Drop for JobOutcomeInner {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
