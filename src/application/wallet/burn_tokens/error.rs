#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenBurnError {
    KycRequired,
    PermissionDenied,
    OrganizationNotFound,
    InvalidInput(String),
    InsufficientFunds,
    KycLoad(String),
    PermissionCheck(String),
    OrganizationLoad(String),
    TaxLoad(String),
    WalletLoad(String),
    WalletCreate(String),
    BurnCreate(String),
    BurnLoad(String),
    LeaderboardLoad(String),
    Connection(String),
}

impl From<diesel::result::Error> for TokenBurnError {
    fn from(error: diesel::result::Error) -> Self {
        Self::BurnCreate(error.to_string())
    }
}
