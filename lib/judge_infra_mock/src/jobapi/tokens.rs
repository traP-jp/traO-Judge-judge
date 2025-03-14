use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OutcomeToken {
    path: Arc<OutcomeTokenInner>,
}

#[derive(Debug)]
struct OutcomeTokenInner {
    pub path: PathBuf,
}

impl OutcomeToken {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: Arc::new(OutcomeTokenInner { path }),
        }
    }

    pub(crate) fn path(&self) -> &PathBuf {
        &self.path.path
    }
}

impl Drop for OutcomeTokenInner {
    fn drop(&mut self) {
        //let _ = std::fs::remove_file(&self.path);
        //let _ = std::fs::remove_dir_all(&self.path);
    }
}

pub struct RegistrationToken {
    pub(crate) _marker: (),
}
