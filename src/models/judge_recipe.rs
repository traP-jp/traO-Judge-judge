pub struct Execution {
    pub optional_info: OptionalInfo,
    pub shell_script_id: uuid::Uuid,
    pub directory_count: f64,
    pub text_resource_count: f64,
    pub one_time_text_count: f64,
}

pub struct ExecutionConfigMap {
    pub text_resource_ids: Vec<uuid::Uuid>,
    pub one_time_text_contents: Vec<String>,
}
pub struct Judge {
    pub judge_id: uuid::Uuid,
    pub test_count: f64,
    pub before_test_execs: Execution,
    pub on_test_execs: Execution,
    pub after_test_execs: Execution,
    pub before_test_config_map: ExecutionConfigMap,
    pub on_test_config_maps: Vec<ExecutionConfigMap>,
    pub after_test_config_map: ExecutionConfigMap,
}

pub struct OptionalInfo {
    pub exec_time: Option<f64>,
    pub memory_size: Option<f64>,
    pub language: Option<String>,
}
