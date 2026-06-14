#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardTokenConfirmationError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NotFound,
    Connection(String),
    Database(String),
}
