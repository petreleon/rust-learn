use crate::domain::access_control::delegation::DelegationRuleError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegatedPermissionError {
    PermissionDenied(String),
    InvalidInput(String),
    NotFound,
    Connection(String),
    Database(String),
}

impl From<DelegationRuleError> for DelegatedPermissionError {
    fn from(error: DelegationRuleError) -> Self {
        match error {
            DelegationRuleError::InvalidInput(message) => Self::InvalidInput(message),
        }
    }
}
