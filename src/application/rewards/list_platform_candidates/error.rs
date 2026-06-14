#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformRewardCandidatesError {
    PermissionDenied(String),
    InvalidStatus(String),
    Connection(String),
    Database(String),
}
