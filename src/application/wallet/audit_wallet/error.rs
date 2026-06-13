#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletAuditError {
    Connection(String),
    Database(String),
}
