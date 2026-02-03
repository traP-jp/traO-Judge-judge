/// WIP
pub struct JobRequest {}

/// `JobResponse` is the output of a process.
pub struct JobResponse {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: u8,
}
