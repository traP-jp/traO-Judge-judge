use crate::model::*;

pub struct NormalJudgeTestcase {
    pub name: String,
    pub input: String,
    pub expected_output: String,
}

pub fn create_normal_judge_procedure(
    time_limit_ms: i64,
    memory_limit_kib: i64,
    testcases: Vec<NormalJudgeTestcase>,
) -> anyhow::Result<procedure::writer_schema::Procedure> {
    todo!()
}
