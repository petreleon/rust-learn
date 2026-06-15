use crate::domain::wallet::deposit::WalletDepositStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletDepositIndexOutput {
    pub intent_id: Option<i64>,
    pub wallet_id: Option<i32>,
    pub transaction_id: Option<i64>,
    pub external_transaction_id: Option<i64>,
    pub internal_transaction_ids: Vec<i64>,
    pub credited: bool,
    pub status: WalletDepositStatus,
}
