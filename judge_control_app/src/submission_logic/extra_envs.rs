pub fn get_extra_envs(
    optional_info: &crate::models::judge_recipe::OptionalInfo,
) -> std::collections::HashMap<String, String> {
    let mut extra_envs = std::collections::HashMap::new();
    if let Some(exec_time) = optional_info.exec_time {
        extra_envs.insert("TIMELIMIT".to_string(), exec_time.to_string());
    }
    if let Some(mem_limit) = optional_info.memory_size {
        extra_envs.insert("MEMLIMIT".to_string(), mem_limit.to_string());
    }
    if let Some(lang) = &optional_info.language {
        extra_envs.insert("LANGUAGE".to_string(), lang.to_string());
    }
    extra_envs
}
