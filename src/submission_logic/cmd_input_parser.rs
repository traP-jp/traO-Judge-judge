use crate::submission_logic::SubmissionInput;
pub mod models;
use models::*;

pub fn get_cmd_input<ExternalAccessKey>(
    submission_input: &SubmissionInput,
) -> CmdInput<ExternalAccessKey> {
    let test_count = submission_input.test_count;
    unimplemented!()
}
