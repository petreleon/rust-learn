#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherRewardCandidateDecisionError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Connection(String),
    Database(String),
}
