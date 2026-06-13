use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentRewardHistoryEntry {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub course_title: String,
    pub event_type: String,
    pub status: String,
    pub approved_amount: Option<String>,
    pub wallet_credit: Option<StudentRewardWalletCredit>,
    pub token_transaction: Option<StudentRewardTokenTransaction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentRewardCandidateRecord {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub course_title: String,
    pub event_type: String,
    pub status: String,
    pub approved_amount: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentRewardWalletCredit {
    pub reward_wallet_credit_record_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: String,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub credited_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentRewardTokenTransaction {
    pub reward_payout_record_id: i64,
    pub payout_transaction_id: i64,
    pub external_transaction_id: i64,
    pub amount: String,
    pub blockchain_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub recorded_at: DateTime<Utc>,
}
