#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardCandidateAuditError {
    PermissionDenied(String),
    NotFound,
    Connection(String),
    Database(String),
}
