pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    OutputLimitExceeded,
    RuntimeError,
    CompileError,
    Critical(CriticalError),
}

pub enum CriticalError {
    WriterError,
    InternalError,
}

pub struct SubmissionOutput {
    pub judge_id: uuid::Uuid,
    pub result: JudgeResult,
}

pub struct TestResult {
    pub status: JudgeStatus,
    pub text: Option<String>,
    pub score: i64,
    pub exec_time: f64,
    pub memory_size: f64,
}

pub type BeforeTestResult = TestResult;
pub type OnTestResult = Vec<TestResult>;
pub type AfterTestResult = TestResult;

pub enum JudgeResult {
    Success(BeforeTestResult, OnTestResult, AfterTestResult),
    BeforeTestFailure(BeforeTestResult),
    OnTestFailure(BeforeTestResult, OnTestResult),
    AfterTestFailure(BeforeTestResult, OnTestResult, AfterTestResult),
}
