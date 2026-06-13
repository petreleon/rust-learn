#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletTokenTaxView {
    pub operation: &'static str,
    pub tax_amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletTokenTaxSettings {
    pub deposit: WalletTokenTaxView,
    pub retire: WalletTokenTaxView,
}
