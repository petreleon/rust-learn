#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardReconciliationError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    NotFound,
    Connection(String),
    Database(String),
}
