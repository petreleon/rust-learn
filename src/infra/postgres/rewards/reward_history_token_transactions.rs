use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::application::rewards::list_reward_history::{
    StudentRewardHistoryError, StudentRewardTokenTransaction,
};
use crate::domain::rewards::token::RewardTokenEventType;

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

pub(super) fn token_transaction(
    row: TokenTransactionRow,
) -> Result<StudentRewardTokenTransaction, StudentRewardHistoryError> {
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
    let event_type = RewardTokenEventType::parse_optional(event)
        .map_err(|error| StudentRewardHistoryError::Database(error.to_string()))?;
    Ok(StudentRewardTokenTransaction {
        reward_payout_record_id: record_id,
        payout_transaction_id: transaction_id,
        external_transaction_id: external_id,
        amount: amount.to_string(),
        blockchain_address: address,
        chain_id,
        contract_address: contract,
        transaction_hash: hash,
        log_index,
        event_type,
        from_address: from,
        to_address: to,
        recorded_at,
    })
}
