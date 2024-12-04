use crate::submission_logic::SubmissionInput;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone)]
pub enum FileLinkRecipe<ExternalAccessKey> {
    TextFile(ExternalAccessKey, usize, PathBuf),
    Directory(PathBuf),
}

#[derive(Clone)]
pub struct CmdInput<ExternalAccessKey> {
    pub cmd: String,
    pub envs: std::collections::HashMap<String, String>,
    pub file_links: std::collections::HashMap<Uuid, FileLinkRecipe<ExternalAccessKey>>,
    

}

pub fn get_cmd_input<T: Ord, ExternalAccessKey>(submission_input: &SubmissionInput<T>) -> CmdInput<ExternalAccessKey> {
    unimplemented!()
}
