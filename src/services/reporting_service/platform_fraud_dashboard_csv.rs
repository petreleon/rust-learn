#[derive(Debug, Clone, Serialize)]
pub struct PlatformWalletReconciliationRow {
    pub wallet_id: i32,
    pub owner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub balance: String,
    pub internal_transaction_count: i64,
    pub external_transaction_count: i64,
    pub reward_record_count: i64,
    pub needs_reconciliation_count: i64,
    pub missing_credit_count: i64,
    pub missing_notification_count: i64,
    pub missing_payout_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformWalletReconciliation {
    pub total_wallets: i64,
    pub total_internal_transactions: i64,
    pub total_external_transactions: i64,
    pub total_reward_records: i64,
    pub total_needs_reconciliation: i64,
    pub wallets: Vec<PlatformWalletReconciliationRow>,
}
