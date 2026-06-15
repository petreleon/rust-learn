use crate::domain::wallet::deposit::WalletDepositStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletDepositIntentView {
    pub operation: &'static str,
    pub id: i64,
    pub status: WalletDepositStatus,
    pub wallet_id: i32,
    pub amount: String,
    pub tax_amount: String,
    pub wallet_delta_on_confirmation: String,
    pub gas_payer: String,
    pub ethereum_address: String,
    pub platform_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}
