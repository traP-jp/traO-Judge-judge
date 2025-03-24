pub const COMPILE_PHASE: &str = "compilePhase";
const TEST_PHASE_PREFIX: &str = "testPhase_";
pub fn test_phase_execution_job_name(core_name: &str) -> String {
    format!("{}{}", TEST_PHASE_PREFIX, core_name)
}
pub const SUMMARY_PHASE: &str = "summaryPhase";
pub mod v0_features {
    // Testcase inputs
    const TESTCASE_INPUT_SUFFIX: &str = "_input";
    pub fn testcase_input_name(core_name: &str) -> String {
        format!("{}{}", core_name, TESTCASE_INPUT_SUFFIX)
    }
    pub fn get_testcase_input_name_from_execution_job_name(
        execution_job_name: &str,
    ) -> anyhow::Result<String> {
        if execution_job_name.starts_with(super::TEST_PHASE_PREFIX) {
            let core_name = &execution_job_name[super::TEST_PHASE_PREFIX.len()..];
            Ok(testcase_input_name(core_name))
        } else {
            Err(anyhow::anyhow!(
                "Execution job name {} does not start with {}",
                execution_job_name,
                super::TEST_PHASE_PREFIX
            ))
        }
    }
    // Testcase expected outputs
    const TESTCASE_EXPECTED_SUFFIX: &str = "_expected";
    pub fn testcase_expected_name(core_name: &str) -> String {
        format!("{}{}", core_name, TESTCASE_EXPECTED_SUFFIX)
    }
    pub fn get_testcase_expected_name_from_execution_job_name(
        execution_job_name: &str,
    ) -> anyhow::Result<String> {
        if execution_job_name.starts_with(super::TEST_PHASE_PREFIX) {
            let core_name = &execution_job_name[super::TEST_PHASE_PREFIX.len()..];
            Ok(testcase_expected_name(core_name))
        } else {
            Err(anyhow::anyhow!(
                "Execution job name {} does not start with {}",
                execution_job_name,
                super::TEST_PHASE_PREFIX
            ))
        }
    }
}
