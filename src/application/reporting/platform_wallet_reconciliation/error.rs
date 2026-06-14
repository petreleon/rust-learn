#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformWalletReconciliationError {
    Connection(String),
    Database(String),
}
