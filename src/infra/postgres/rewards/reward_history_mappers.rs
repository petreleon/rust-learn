use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::application::rewards::list_reward_history::{
    StudentRewardCandidateRecord, StudentRewardHistoryError, StudentRewardTokenTransaction,
    StudentRewardWalletCredit,
};
use crate::models::reward_candidate::RewardCandidate;

pub(super) type WalletCreditRow = (
    i64,
    i32,
    i64,
    i64,
    Option<i64>,
    Option<DateTime<Utc>>,
    DateTime<Utc>,
    BigDecimal,
);

pub(super) type TokenTransactionRow = (
    i64,
    i64,
    i64,
    DateTime<Utc>,
    BigDecimal,
    String,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<String>,
);

pub(super) fn candidate_record(
    candidate: RewardCandidate,
    course_title: String,
) -> StudentRewardCandidateRecord {
    StudentRewardCandidateRecord {
        reward_candidate_id: candidate.id,
        course_id: candidate.course_id,
        course_title,
        event_type: candidate.event_type,
        status: candidate.status,
        approved_amount: candidate.approved_amount.map(|amount| amount.to_string()),
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    }
}

pub(super) fn wallet_credit(row: WalletCreditRow) -> StudentRewardWalletCredit {
    let (
        id,
        wallet_id,
        transaction_id,
        internal_id,
        notification_id,
        notified_at,
        credited_at,
        amount,
    ) = row;
    StudentRewardWalletCredit {
        reward_wallet_credit_record_id: id,
        wallet_id,
        transaction_id,
        internal_transaction_id: internal_id,
        amount: amount.to_string(),
        notification_id,
        notified_at,
        credited_at,
    }
}

pub(super) fn token_transaction(row: TokenTransactionRow) -> StudentRewardTokenTransaction {
    let (
        record_id,
        transaction_id,
        external_id,
        recorded_at,
        amount,
        address,
        chain_id,
        contract,
        hash,
        log_index,
        event,
        from,
        to,
    ) = row;
    StudentRewardTokenTransaction {
        reward_payout_record_id: record_id,
        payout_transaction_id: transaction_id,
        external_transaction_id: external_id,
        amount: amount.to_string(),
        blockchain_address: address,
        chain_id,
        contract_address: contract,
        transaction_hash: hash,
        log_index,
        event_type: event,
        from_address: from,
        to_address: to,
        recorded_at,
    }
}

pub(super) fn map_reward_history_error(error: diesel::result::Error) -> StudentRewardHistoryError {
    StudentRewardHistoryError::Database(error.to_string())
}
