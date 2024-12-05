use crate::submission_logic::SubmissionInput;
use std::path::PathBuf;
use uuid::Uuid;
pub mod models;
use models::*;

pub fn get_cmd_input<T: Ord, ExternalAccessKey>(
    submission_input: &SubmissionInput<T>,
) -> CmdInput<ExternalAccessKey> {
    let test_count = submission_input.test_count;

    unimplemented!()
}
