use bigdecimal::BigDecimal;

use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexError, WalletDepositIndexOutput,
    WalletDepositIndexStore,
};
use crate::domain::wallet::deposit::wallet_deposit_event_type_is_supported;

pub async fn index_observed_deposit(
    store: &mut impl WalletDepositIndexStore,
    event: ObservedWalletDepositEvent,
) -> Result<WalletDepositIndexOutput, WalletDepositIndexError> {
    validate_observed_wallet_deposit_event(&event)?;
    store.index_observed_deposit(event).await
}

fn validate_observed_wallet_deposit_event(
    event: &ObservedWalletDepositEvent,
) -> Result<(), WalletDepositIndexError> {
    if event.chain_id <= 0 {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed chain_id must be positive".to_string(),
        ));
    }
    if event.transaction_hash.trim().is_empty() {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed transaction_hash is required".to_string(),
        ));
    }
    if event.log_index < 0 {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed log_index cannot be negative".to_string(),
        ));
    }
    if event.contract_address.trim().is_empty() {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed contract_address is required".to_string(),
        ));
    }
    if event.from_address.trim().is_empty() {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed from_address is required".to_string(),
        ));
    }
    if event.to_address.trim().is_empty() {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed to_address is required".to_string(),
        ));
    }
    if event.amount <= BigDecimal::from(0) {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed amount must be positive".to_string(),
        ));
    }
    if !wallet_deposit_event_type_is_supported(event.event_type.as_str()) {
        return Err(WalletDepositIndexError::InvalidInput(
            "observed event_type must be 'import' or 'transfer'".to_string(),
        ));
    }

    Ok(())
}
