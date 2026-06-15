#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RewardCandidateError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidStatus(String),
    NotFound,
    Database(String),
}
