#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletRetirementView {
    pub operation: &'static str,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub internal_transaction_ids: Vec<i64>,
    pub amount: String,
    pub tax_amount: String,
    pub wallet_delta: String,
    pub gas_payer: String,
    pub ethereum_address: String,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}
