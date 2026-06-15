use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::rewards::list_reward_history::{
    StudentRewardHistoryEntry, StudentRewardHistoryQuery, StudentRewardTokenTransaction,
    StudentRewardWalletCredit,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct StudentRewardHistoryRequest {
    pub course_id: Option<i32>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StudentRewardHistoryEntryResponse {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub course_title: String,
    pub event_type: String,
    pub status: String,
    pub approved_amount: Option<String>,
    pub wallet_credit: Option<StudentRewardWalletCreditResponse>,
    pub token_transaction: Option<StudentRewardTokenTransactionResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StudentRewardWalletCreditResponse {
    pub reward_wallet_credit_record_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: String,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub credited_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StudentRewardTokenTransactionResponse {
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

impl From<StudentRewardHistoryRequest> for StudentRewardHistoryQuery {
    fn from(request: StudentRewardHistoryRequest) -> Self {
        Self {
            course_id: request.course_id,
            status: request.status,
            limit: request.limit,
            offset: request.offset,
        }
    }
}

impl From<StudentRewardHistoryEntry> for StudentRewardHistoryEntryResponse {
    fn from(entry: StudentRewardHistoryEntry) -> Self {
        Self {
            reward_candidate_id: entry.reward_candidate_id,
            course_id: entry.course_id,
            course_title: entry.course_title,
            event_type: entry.event_type.as_str().to_string(),
            status: entry.status.as_str().to_string(),
            approved_amount: entry.approved_amount,
            wallet_credit: entry
                .wallet_credit
                .map(StudentRewardWalletCreditResponse::from),
            token_transaction: entry
                .token_transaction
                .map(StudentRewardTokenTransactionResponse::from),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

impl From<StudentRewardWalletCredit> for StudentRewardWalletCreditResponse {
    fn from(credit: StudentRewardWalletCredit) -> Self {
        Self {
            reward_wallet_credit_record_id: credit.reward_wallet_credit_record_id,
            wallet_id: credit.wallet_id,
            transaction_id: credit.transaction_id,
            internal_transaction_id: credit.internal_transaction_id,
            amount: credit.amount,
            notification_id: credit.notification_id,
            notified_at: credit.notified_at,
            credited_at: credit.credited_at,
        }
    }
}

impl From<StudentRewardTokenTransaction> for StudentRewardTokenTransactionResponse {
    fn from(transaction: StudentRewardTokenTransaction) -> Self {
        Self {
            reward_payout_record_id: transaction.reward_payout_record_id,
            payout_transaction_id: transaction.payout_transaction_id,
            external_transaction_id: transaction.external_transaction_id,
            amount: transaction.amount,
            blockchain_address: transaction.blockchain_address,
            chain_id: transaction.chain_id,
            contract_address: transaction.contract_address,
            transaction_hash: transaction.transaction_hash,
            log_index: transaction.log_index,
            event_type: transaction.event_type,
            from_address: transaction.from_address,
            to_address: transaction.to_address,
            recorded_at: transaction.recorded_at,
        }
    }
}
