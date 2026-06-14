#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletTokenTaxOperation {
    Deposit,
    Retire,
}

impl WalletTokenTaxOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deposit => "deposit",
            Self::Retire => "retire",
        }
    }
}
