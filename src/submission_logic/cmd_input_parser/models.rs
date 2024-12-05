use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone)]
pub struct TextFileRecipe<ExternalAccessKey> {
    pub text_resource_id: ExternalAccessKey,
    pub replica: usize,
    pub path: PathBuf,
    pub cache: bool,
}

#[derive(Clone)]
pub struct DirectoryRecipe {
    pub path: PathBuf,
}

#[derive(Clone)]
pub enum FileLinkRecipe<ExternalAccessKey> {
    TextFile(TextFileRecipe<ExternalAccessKey>),
    Directory(DirectoryRecipe),
}

#[derive(Clone)]
pub struct SingleExecutionConfig {
    pub file_ids: Vec<Uuid>,
    pub hook_sh_id: Uuid,
    pub cmd: String,
    pub envs: std::collections::HashMap<String, String>,
}

#[derive(Clone)]
pub struct CmdInput<ExternalAccessKey> {
    pub file_links: std::collections::HashMap<Uuid, FileLinkRecipe<ExternalAccessKey>>,
}
