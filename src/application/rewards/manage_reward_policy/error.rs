#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardPolicyError {
    PermissionDenied(String),
    InvalidInput(String),
    NotFound,
    Connection(String),
    Database(String),
}
