use crate::model::*;

pub struct NormalJudgeTestcase {
    pub input: String,
    pub expected_output: String,
}

pub fn create_normal_judge_procedure(
    testcases: Vec<NormalJudgeTestcase>,
) -> anyhow::Result<procedure::writer_schema::Procedure> {
    todo!()
}
