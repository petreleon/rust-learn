use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct WalletDepositIntentDraft {
    pub amount: BigDecimal,
    pub ethereum_address: String,
    pub platform_address: String,
    pub gas_payer: String,
    pub tax_amount: BigDecimal,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}
