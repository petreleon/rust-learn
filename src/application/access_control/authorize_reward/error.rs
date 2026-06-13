use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardAuthorizationError {
    PermissionCheck(String),
}

impl fmt::Display for RewardAuthorizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionCheck(message) => formatter.write_str(message),
        }
    }
}
