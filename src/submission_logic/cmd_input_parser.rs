use crate::submission_logic::SubmissionInput;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone)]
pub enum FileLinkRecipe<ExternalAccessKey> {
    TextFile(ExternalAccessKey, usize, Uuid),
    Directory(Uuid),
}

#[derive(Clone)]
pub struct CmdInput<ExternalAccessKey> {
    pub cmd: String,
    pub envs: std::collections::HashMap<String, String>,
    pub file_links: std::collections::HashMap<PathBuf, FileLinkRecipe<ExternalAccessKey>>,

}

pub fn get_cmd_input<T: Ord, ExternalAccessKey>(submission_input: &SubmissionInput<T>) -> CmdInput<ExternalAccessKey> {
    unimplemented!()
}
