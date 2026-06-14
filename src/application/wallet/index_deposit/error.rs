#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletDepositIndexError {
    InvalidInput(String),
    InsufficientFunds,
    Database(String),
    Connection(String),
}
