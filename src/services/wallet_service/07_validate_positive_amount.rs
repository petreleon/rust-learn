fn validate_positive_amount(
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

fn validate_non_negative_amount(
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

fn validate_transfer_request_addresses(
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

fn validate_external_transaction_fields(
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

fn validate_observed_wallet_deposit_event(
    event: &ObservedWalletDepositEvent,
) -> Result<(), WalletTokenTransferError> {
    if event.chain_id <= 0 {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed chain_id must be positive".to_string(),
        ));
    }
    if event.transaction_hash.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed transaction_hash is required".to_string(),
        ));
    }
    if event.log_index < 0 {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed log_index cannot be negative".to_string(),
        ));
    }
    if event.contract_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed contract_address is required".to_string(),
        ));
    }
    if event.from_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed from_address is required".to_string(),
        ));
    }
    if event.to_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed to_address is required".to_string(),
        ));
    }
    validate_positive_amount(&event.amount, "observed amount")?;
    if ![TOKEN_TRANSFER_EVENT_IMPORT, TOKEN_TRANSFER_EVENT_TRANSFER]
        .contains(&event.event_type.as_str())
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed event_type must be 'import' or 'transfer'".to_string(),
        ));
    }

    Ok(())
}
