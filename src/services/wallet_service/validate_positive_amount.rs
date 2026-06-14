use bigdecimal::BigDecimal;

use super::support::{WalletTokenTransferError, WalletTokenTransferRequest};

pub(super) fn validate_positive_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletTokenTransferError> {
    if amount <= &BigDecimal::from(0) {
        return Err(WalletTokenTransferError::InvalidInput(format!(
            "{} must be positive",
            field_name
        )));
    }
    Ok(())
}

pub(super) fn validate_non_negative_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletTokenTransferError> {
    if amount < &BigDecimal::from(0) {
        return Err(WalletTokenTransferError::InvalidInput(format!(
            "{} cannot be negative",
            field_name
        )));
    }
    Ok(())
}

pub(super) fn validate_transfer_request_addresses(
    request: &WalletTokenTransferRequest,
) -> Result<(), WalletTokenTransferError> {
    if request.ethereum_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "ethereum_address is required".to_string(),
        ));
    }

    if request
        .platform_address
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "platform_address cannot be empty when provided".to_string(),
        ));
    }

    Ok(())
}

pub(super) fn validate_external_transaction_fields(
    request: &WalletTokenTransferRequest,
) -> Result<(), WalletTokenTransferError> {
    if request
        .chain_id
        .map(|chain_id| chain_id <= 0)
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "chain_id must be positive when provided".to_string(),
        ));
    }
    if request
        .log_index
        .map(|log_index| log_index < 0)
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    if request
        .contract_address
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "contract_address cannot be empty when provided".to_string(),
        ));
    }
    if request
        .transaction_hash
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "transaction_hash cannot be empty when provided".to_string(),
        ));
    }

    Ok(())
}
