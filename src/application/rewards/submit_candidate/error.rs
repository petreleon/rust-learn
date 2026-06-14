#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardCandidateSubmissionError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidStatus(String),
    NotFound,
    Connection(String),
    Database(String),
}
