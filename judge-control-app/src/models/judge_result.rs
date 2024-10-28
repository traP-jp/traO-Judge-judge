pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    OutputLimitExceeded,
    RuntimeError,
    CompileError,
    InternalError,
}

pub struct SubmitResult {
    pub judge_id: uuid::Uuid,
    pub test_results: Vec<TestResult>,
    pub total_result: Box<TestResult>,
}

pub struct TestResult {
    pub status: JudgeStatus,
    pub text: Option<String>,
    pub score: f64,
    pub exec_time: f64,
    pub memory_size: f64,
}
