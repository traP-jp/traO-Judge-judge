use super::super::models::Phase;
use crate::models::judge_recipe::SubmissionInput;

#[derive(PartialEq, Eq, Clone)]
pub struct JudgePriority {
    pub posted_at: chrono::NaiveDateTime,
    pub phase: Phase,
}

impl PartialOrd for JudgePriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.posted_at.cmp(&other.posted_at))
    }
}

impl Ord for JudgePriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.posted_at.cmp(&other.posted_at)
    }
}

impl From<(SubmissionInput, Phase)> for JudgePriority {
    fn from(inputs: (SubmissionInput, Phase)) -> Self {
        Self {
            posted_at: inputs.0.posted_at,
            phase: inputs.1,
        }
    }
}
