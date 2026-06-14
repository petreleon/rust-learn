#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletDepositIntentError {
    KycRequired,
    InvalidInput(String),
    KycLoad(String),
    TaxLoad(String),
    ConfigurationLoad(String),
    WalletLoad(String),
    WalletCreate(String),
    DepositIntentCreate(String),
    Connection(String),
}
