use bigdecimal::BigDecimal;

use crate::application::wallet::retire_tokens::{
    WalletRetirementDraft, WalletRetirementError, WalletRetirementGasPayer,
    WalletRetirementRequest, WalletRetirementStore, WalletRetirementView,
};

pub async fn retire_tokens(
    store: &mut impl WalletRetirementStore,
    user_id: i32,
    request: WalletRetirementRequest,
) -> Result<WalletRetirementView, WalletRetirementError> {
    if !store.user_kyc_verified(user_id).await? {
        return Err(WalletRetirementError::KycRequired);
    }

    validate_request(&request)?;
    let gas_payer = WalletRetirementGasPayer::parse(&request.gas_payer)?;
    let tax_amount = match gas_payer {
        WalletRetirementGasPayer::User => BigDecimal::from(0),
        WalletRetirementGasPayer::Platform => store.load_platform_retire_tax().await?,
    };
    validate_non_negative_amount(&tax_amount, "stored tax amount")?;

    let draft = WalletRetirementDraft {
        amount: request.amount,
        tax_amount,
        gas_payer: gas_payer.as_str().to_string(),
        ethereum_address: request.ethereum_address.trim().to_string(),
        platform_address: request
            .platform_address
            .as_deref()
            .map(|value| value.trim().to_string()),
        chain_id: request.chain_id,
        contract_address: request
            .contract_address
            .as_deref()
            .map(|value| value.trim().to_string()),
        transaction_hash: request
            .transaction_hash
            .as_deref()
            .map(|value| value.trim().to_string()),
        log_index: request.log_index,
        wallet_provider: gas_payer.wallet_provider().to_string(),
        metamask_required: gas_payer.metamask_required(),
        wallet_action: gas_payer.wallet_action().to_string(),
    };

    store.retire_tokens(user_id, draft).await
}

fn validate_request(request: &WalletRetirementRequest) -> Result<(), WalletRetirementError> {
    validate_positive_amount(&request.amount, "amount")?;
    if request.ethereum_address.trim().is_empty() {
        return Err(WalletRetirementError::InvalidInput(
            "ethereum_address is required".to_string(),
        ));
    }
    if request
        .platform_address
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletRetirementError::InvalidInput(
            "platform_address cannot be empty when provided".to_string(),
        ));
    }
    validate_external_transaction_fields(request)
}

fn validate_positive_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletRetirementError> {
    if amount <= &BigDecimal::from(0) {
        return Err(WalletRetirementError::InvalidInput(format!(
            "{} must be positive",
            field_name
        )));
    }
    Ok(())
}

fn validate_non_negative_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletRetirementError> {
    if amount < &BigDecimal::from(0) {
        return Err(WalletRetirementError::InvalidInput(format!(
            "{} cannot be negative",
            field_name
        )));
    }
    Ok(())
}

fn validate_external_transaction_fields(
    request: &WalletRetirementRequest,
) -> Result<(), WalletRetirementError> {
    if request
        .chain_id
        .map(|chain_id| chain_id <= 0)
        .unwrap_or(false)
    {
        return Err(WalletRetirementError::InvalidInput(
            "chain_id must be positive when provided".to_string(),
        ));
    }
    if request
        .log_index
        .map(|log_index| log_index < 0)
        .unwrap_or(false)
    {
        return Err(WalletRetirementError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    validate_optional_text(request.contract_address.as_ref(), "contract_address")?;
    validate_optional_text(request.transaction_hash.as_ref(), "transaction_hash")
}

fn validate_optional_text(
    value: Option<&String>,
    field_name: &str,
) -> Result<(), WalletRetirementError> {
    if value.map(|value| value.trim().is_empty()).unwrap_or(false) {
        return Err(WalletRetirementError::InvalidInput(format!(
            "{} cannot be empty when provided",
            field_name
        )));
    }
    Ok(())
}
