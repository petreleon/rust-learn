#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardPayoutPlanError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    Connection(String),
    Database(String),
}
