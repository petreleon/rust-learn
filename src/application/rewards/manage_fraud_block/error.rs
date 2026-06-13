#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardFraudBlockError {
    PermissionDenied(String),
    InvalidInput(String),
    NotFound,
    Connection(String),
    Database(String),
}
