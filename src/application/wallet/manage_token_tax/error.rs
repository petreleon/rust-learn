#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletTokenTaxError {
    PermissionDenied,
    InvalidInput(String),
    PermissionCheck(String),
    TaxLoad(String),
    TaxStore(String),
    Connection(String),
}
