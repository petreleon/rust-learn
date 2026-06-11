use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, external_transactions, internal_transactions, reward_candidates,
    reward_payout_records, reward_wallet_credit_records,
};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_ADJUSTED, REWARD_STATUS_AMOUNT_APPROVED,
    REWARD_STATUS_AMOUNT_REJECTED, REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED,
    REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::repositories::course_repository::user_permission_course_request;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

const DEFAULT_STUDENT_REWARD_HISTORY_LIMIT: i64 = 50;
const MAX_STUDENT_REWARD_HISTORY_LIMIT: i64 = 100;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct StudentRewardHistoryRequest {
    pub course_id: Option<i32>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl StudentRewardHistoryRequest {
    fn limit(&self) -> i64 {
        self.limit
            .unwrap_or(DEFAULT_STUDENT_REWARD_HISTORY_LIMIT)
            .clamp(1, MAX_STUDENT_REWARD_HISTORY_LIMIT)
    }

    fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum StudentRewardHistoryError {
    InvalidInput(String),
    Database(String),
}

impl From<diesel::result::Error> for StudentRewardHistoryError {
    fn from(error: diesel::result::Error) -> Self {
        StudentRewardHistoryError::Database(error.to_string())
    }
}
