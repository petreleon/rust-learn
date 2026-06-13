#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardWalletCreditError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    NotFound,
    Connection(String),
    Database(String),
}
