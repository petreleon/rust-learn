#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherRewardCandidateDecisionCommand {
    pub status: String,
    pub decision_reason: Option<String>,
}
