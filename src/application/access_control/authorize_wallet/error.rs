use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletAuthorizationError {
    PermissionCheck(String),
    Connection(String),
}

impl fmt::Display for WalletAuthorizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionCheck(message) | Self::Connection(message) => {
                formatter.write_str(message)
            }
        }
    }
}
