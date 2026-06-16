use chrono::{DateTime, Utc};

use crate::domain::rewards::candidate::reconciliation::RewardReconciliationStatus;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::token::RewardTokenEventType;
use crate::domain::wallet::deposit::WalletDepositStatus;
use crate::domain::wallet::owner::WalletOwnerType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletAudit {
    pub wallet: WalletAuditWallet,
    pub internal_transactions: Vec<WalletInternalTransactionAudit>,
    pub external_transactions: Vec<WalletExternalTransactionAudit>,
    pub deposit_intents: Vec<WalletDepositIntentAudit>,
    pub reward_records: Vec<WalletRewardRecordAudit>,
    pub compensation_records: Vec<WalletCompensationRecordAudit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletAuditWallet {
    pub id: i32,
    pub owner_type: WalletOwnerType,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletInternalTransactionAudit {
    pub internal_transaction_id: i64,
    pub transaction_id: i64,
    pub transaction_type: String,
    pub amount: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletExternalTransactionAudit {
    pub external_transaction_id: i64,
    pub transaction_id: i64,
    pub reward_candidate_id: Option<i64>,
    pub amount: String,
    pub blockchain_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<RewardTokenEventType>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletDepositIntentAudit {
    pub id: i64,
    pub user_id: i32,
    pub wallet_id: i32,
    pub ethereum_address: String,
    pub platform_address: String,
    pub amount: String,
    pub tax_amount: String,
    pub gas_payer: String,
    pub status: WalletDepositStatus,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<RewardTokenEventType>,
    pub external_transaction_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub credited_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletRewardRecordAudit {
    pub reward_candidate_id: i64,
    pub candidate_status: RewardCandidateStatus,
    pub reconciliation_status: RewardReconciliationStatus,
    pub approved_amount: Option<String>,
    pub wallet_credit_record_id: Option<i64>,
    pub wallet_credit_transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub payout_record_id: Option<i64>,
    pub payout_transaction_id: Option<i64>,
    pub external_transaction_id: Option<i64>,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletCompensationRecordAudit {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: String,
    pub reason: String,
    pub idempotency_key: String,
    pub created_by_user_id: i32,
    pub created_at: DateTime<Utc>,
}
