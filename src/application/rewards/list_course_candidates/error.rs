#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseRewardCandidatesError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Connection(String),
    Database(String),
}
