use crate::application::wallet::audit_wallet::{WalletAuditError, WalletExternalTransactionAudit};
use crate::domain::rewards::token::RewardTokenEventType;

pub(super) type WalletExternalTransactionRow = (
    i64,
    i64,
    bigdecimal::BigDecimal,
    String,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<String>,
);

pub(super) type RewardExternalTransactionRow = (
    i64,
    i64,
    i64,
    bigdecimal::BigDecimal,
    String,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<String>,
);

pub(super) fn reward_external_transaction_audit(
    row: RewardExternalTransactionRow,
) -> Result<WalletExternalTransactionAudit, WalletAuditError> {
    let (
        reward_candidate_id,
        transaction_id,
        external_transaction_id,
        amount,
        blockchain_address,
        chain_id,
        contract_address,
        transaction_hash,
        log_index,
        event_type,
        from_address,
        to_address,
    ) = row;

    let event_type = RewardTokenEventType::parse_optional(event_type)
        .map_err(|error| WalletAuditError::AuditLoad(error.to_string()))?;

    Ok(WalletExternalTransactionAudit {
        external_transaction_id,
        transaction_id,
        reward_candidate_id: Some(reward_candidate_id),
        amount: amount.to_string(),
        blockchain_address,
        chain_id,
        contract_address,
        transaction_hash,
        log_index,
        event_type,
        from_address,
        to_address,
    })
}

pub(super) fn wallet_external_transaction_audit(
    row: WalletExternalTransactionRow,
) -> Result<WalletExternalTransactionAudit, WalletAuditError> {
    let (
        transaction_id,
        external_transaction_id,
        amount,
        blockchain_address,
        chain_id,
        contract_address,
        transaction_hash,
        log_index,
        event_type,
        from_address,
        to_address,
    ) = row;

    let event_type = RewardTokenEventType::parse_optional(event_type)
        .map_err(|error| WalletAuditError::AuditLoad(error.to_string()))?;

    Ok(WalletExternalTransactionAudit {
        external_transaction_id,
        transaction_id,
        reward_candidate_id: None,
        amount: amount.to_string(),
        blockchain_address,
        chain_id,
        contract_address,
        transaction_hash,
        log_index,
        event_type,
        from_address,
        to_address,
    })
}
