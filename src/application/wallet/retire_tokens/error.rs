#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletRetirementError {
    KycRequired,
    InvalidInput(String),
    InsufficientFunds,
    KycLoad(String),
    TaxLoad(String),
    WalletLoad(String),
    WalletCreate(String),
    RetirementCreate(String),
    Connection(String),
}
