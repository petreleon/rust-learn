use bigdecimal::BigDecimal;

use crate::application::wallet::create_deposit_intent::{
    WalletDepositGasPayer, WalletDepositIntentDraft, WalletDepositIntentError,
    WalletDepositIntentRequest, WalletDepositIntentStore, WalletDepositIntentView,
};

pub async fn create_deposit_intent(
    store: &mut impl WalletDepositIntentStore,
    user_id: i32,
    request: WalletDepositIntentRequest,
) -> Result<WalletDepositIntentView, WalletDepositIntentError> {
    if !store.user_kyc_verified(user_id).await? {
        return Err(WalletDepositIntentError::KycRequired);
    }

    validate_request(&request)?;
    let gas_payer = WalletDepositGasPayer::parse(&request.gas_payer)?;
    let tax_amount = match gas_payer {
        WalletDepositGasPayer::User => BigDecimal::from(0),
        WalletDepositGasPayer::Platform => store.load_platform_deposit_tax().await?,
    };
    validate_non_negative_amount(&tax_amount, "stored tax amount")?;

    if tax_amount > request.amount {
        return Err(WalletDepositIntentError::InvalidInput(
            "deposit amount must be greater than or equal to the platform-paid gas tax".to_string(),
        ));
    }

    let platform_address = store.configured_deposit_platform_address(gas_payer).await?;
    if let Some(requested_platform_address) = request.platform_address.as_ref() {
        if normalize_address(requested_platform_address) != platform_address {
            return Err(WalletDepositIntentError::InvalidInput(
                "platform_address does not match the configured deposit receiver".to_string(),
            ));
        }
    }

    let draft = WalletDepositIntentDraft {
        amount: request.amount,
        ethereum_address: normalize_address(&request.ethereum_address),
        platform_address,
        gas_payer: gas_payer.as_str().to_string(),
        tax_amount,
        chain_id: request.chain_id,
        contract_address: request.contract_address.as_deref().map(normalize_address),
        transaction_hash: request
            .transaction_hash
            .as_deref()
            .map(|value| value.trim().to_ascii_lowercase()),
        log_index: request.log_index,
        wallet_provider: gas_payer.wallet_provider().to_string(),
        metamask_required: gas_payer.metamask_required(),
        wallet_action: gas_payer.wallet_action().to_string(),
    };

    store.create_deposit_intent(user_id, draft).await
}

fn validate_request(request: &WalletDepositIntentRequest) -> Result<(), WalletDepositIntentError> {
    validate_positive_amount(&request.amount, "amount")?;
    if request.ethereum_address.trim().is_empty() {
        return Err(WalletDepositIntentError::InvalidInput(
            "ethereum_address is required".to_string(),
        ));
    }
    if request
        .platform_address
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletDepositIntentError::InvalidInput(
            "platform_address cannot be empty when provided".to_string(),
        ));
    }
    validate_external_transaction_fields(request)
}

fn validate_positive_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletDepositIntentError> {
    if amount <= &BigDecimal::from(0) {
        return Err(WalletDepositIntentError::InvalidInput(format!(
            "{} must be positive",
            field_name
        )));
    }
    Ok(())
}

fn validate_non_negative_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletDepositIntentError> {
    if amount < &BigDecimal::from(0) {
        return Err(WalletDepositIntentError::InvalidInput(format!(
            "{} cannot be negative",
            field_name
        )));
    }
    Ok(())
}

fn validate_external_transaction_fields(
    request: &WalletDepositIntentRequest,
) -> Result<(), WalletDepositIntentError> {
    if request
        .chain_id
        .map(|chain_id| chain_id <= 0)
        .unwrap_or(false)
    {
        return Err(WalletDepositIntentError::InvalidInput(
            "chain_id must be positive when provided".to_string(),
        ));
    }
    if request
        .log_index
        .map(|log_index| log_index < 0)
        .unwrap_or(false)
    {
        return Err(WalletDepositIntentError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    validate_optional_text(request.contract_address.as_ref(), "contract_address")?;
    validate_optional_text(request.transaction_hash.as_ref(), "transaction_hash")
}

fn validate_optional_text(
    value: Option<&String>,
    field_name: &str,
) -> Result<(), WalletDepositIntentError> {
    if value.map(|value| value.trim().is_empty()).unwrap_or(false) {
        return Err(WalletDepositIntentError::InvalidInput(format!(
            "{} cannot be empty when provided",
            field_name
        )));
    }
    Ok(())
}

fn normalize_address(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
