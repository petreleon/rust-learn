#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardCompensationError {
    PermissionDenied(String),
    InvalidInput(String),
    InsufficientFunds,
    NotFound,
    Connection(String),
    Database(String),
}
