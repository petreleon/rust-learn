#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardAmountDecisionError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidStatus(String),
    NotFound,
    Connection(String),
    Database(String),
}
