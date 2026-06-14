#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardWalletCreditNotificationError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NotFound,
    Connection(String),
    Database(String),
}
