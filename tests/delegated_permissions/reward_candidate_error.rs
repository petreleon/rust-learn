#[derive(Debug, PartialEq, Eq)]
enum RewardCandidateError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidStatus(String),
    NotFound,
    Database(String),
}
