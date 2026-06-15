#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardCandidateAuditError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Connection(String),
    Database(String),
}
