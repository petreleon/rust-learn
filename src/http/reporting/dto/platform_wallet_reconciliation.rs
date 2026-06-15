use serde::Serialize;

use crate::application::reporting::platform_wallet_reconciliation::{
    PlatformWalletReconciliationOutput, PlatformWalletReconciliationRowOutput,
};

#[derive(Debug, Clone, Serialize)]
pub struct PlatformWalletReconciliationResponse {
    pub total_wallets: i64,
    pub total_internal_transactions: i64,
    pub total_external_transactions: i64,
    pub total_reward_records: i64,
    pub total_needs_reconciliation: i64,
    pub wallets: Vec<PlatformWalletReconciliationRowResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformWalletReconciliationRowResponse {
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

impl From<PlatformWalletReconciliationOutput> for PlatformWalletReconciliationResponse {
    fn from(output: PlatformWalletReconciliationOutput) -> Self {
        Self {
            total_wallets: output.total_wallets,
            total_internal_transactions: output.total_internal_transactions,
            total_external_transactions: output.total_external_transactions,
            total_reward_records: output.total_reward_records,
            total_needs_reconciliation: output.total_needs_reconciliation,
            wallets: output.wallets.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PlatformWalletReconciliationRowOutput> for PlatformWalletReconciliationRowResponse {
    fn from(row: PlatformWalletReconciliationRowOutput) -> Self {
        Self {
            wallet_id: row.wallet_id,
            owner_type: row.owner_type.as_str().to_string(),
            user_id: row.user_id,
            organization_id: row.organization_id,
            balance: row.balance,
            internal_transaction_count: row.internal_transaction_count,
            external_transaction_count: row.external_transaction_count,
            reward_record_count: row.reward_record_count,
            needs_reconciliation_count: row.needs_reconciliation_count,
            missing_credit_count: row.missing_credit_count,
            missing_notification_count: row.missing_notification_count,
            missing_payout_count: row.missing_payout_count,
        }
    }
}
