#[derive(Clone)]
pub struct Execution {
    pub optional_info: OptionalInfo,
    pub shell_script_id: uuid::Uuid,
    pub directory_count: i64,
    pub text_resource_count: i64,
    pub one_time_text_count: i64,
}

#[derive(Clone)]
pub struct ExecutionConfigMap {
    pub text_resource_ids: Vec<uuid::Uuid>,
    pub one_time_text_contents: Vec<String>,
}

#[derive(Clone)]
pub struct SubmissionInput {
    pub judge_id: uuid::Uuid,
    pub test_count: i64,
    pub before_test_execs: Execution,
    pub on_test_execs: Execution,
    pub after_test_execs: Execution,
    pub before_test_config_map: ExecutionConfigMap,
    pub on_test_config_maps: Vec<ExecutionConfigMap>,
    pub after_test_config_map: ExecutionConfigMap,
    pub posted_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct OptionalInfo {
    pub exec_time: Option<f64>,
    pub memory_size: Option<f64>,
    pub language: Option<String>,
}
