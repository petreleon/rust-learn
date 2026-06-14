use crate::domain::kyc::submission::KycRuleError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KycError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    NotFound,
    Connection(String),
    Database(String),
}

impl From<KycRuleError> for KycError {
    fn from(error: KycRuleError) -> Self {
        match error {
            KycRuleError::InvalidInput(message) => Self::InvalidInput(message),
            KycRuleError::InvalidTransition(message) => Self::InvalidTransition(message),
        }
    }
}
