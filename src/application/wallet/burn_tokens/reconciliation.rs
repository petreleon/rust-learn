use crate::application::wallet::burn_tokens::{
    TokenBurnError, TokenBurnReconciliationCommand, TokenBurnStore, TokenBurnView,
};

pub async fn list_token_burn_reconciliation_queue(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
) -> Result<Vec<TokenBurnView>, TokenBurnError> {
    ensure_can_reconcile(store, actor_user_id).await?;
    store.list_reconciliation_burns().await
}

pub async fn list_failed_token_burns(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
) -> Result<Vec<TokenBurnView>, TokenBurnError> {
    ensure_can_reconcile(store, actor_user_id).await?;
    store.list_failed_burns().await
}

pub async fn reconcile_token_burn(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
    burn_request_id: i64,
    command: TokenBurnReconciliationCommand,
) -> Result<TokenBurnView, TokenBurnError> {
    ensure_can_reconcile(store, actor_user_id).await?;
    validate_reconciliation_command(&command)?;
    store.reconcile_burn_request(burn_request_id, command).await
}

async fn ensure_can_reconcile(
    store: &mut impl TokenBurnStore,
    actor_user_id: i32,
) -> Result<(), TokenBurnError> {
    if store.can_reconcile_token_burns(actor_user_id).await? {
        Ok(())
    } else {
        Err(TokenBurnError::PermissionDenied)
    }
}

fn validate_reconciliation_command(
    command: &TokenBurnReconciliationCommand,
) -> Result<(), TokenBurnError> {
    validate_optional_text(command.ethereum_address.as_ref(), "ethereum_address")?;
    validate_optional_text(command.platform_address.as_ref(), "platform_address")?;
    validate_optional_text(command.contract_address.as_ref(), "contract_address")?;
    validate_optional_text(command.transaction_hash.as_ref(), "transaction_hash")?;
    validate_optional_text(command.error_message.as_ref(), "error_message")?;

    if command.chain_id.map(|value| value <= 0).unwrap_or(false) {
        return Err(TokenBurnError::InvalidInput(
            "chain_id must be positive when provided".to_string(),
        ));
    }
    if command.log_index.map(|value| value < 0).unwrap_or(false) {
        return Err(TokenBurnError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    if command.mark_failed.unwrap_or(false) && command.error_message.is_none() {
        return Err(TokenBurnError::InvalidInput(
            "error_message is required when marking a burn as failed".to_string(),
        ));
    }
    if command.transaction_hash.is_some()
        && command
            .ethereum_address
            .as_ref()
            .or(command.platform_address.as_ref())
            .is_none()
    {
        return Err(TokenBurnError::InvalidInput(
            "ethereum_address or platform_address is required with transaction_hash".to_string(),
        ));
    }
    Ok(())
}

fn validate_optional_text(value: Option<&String>, field: &str) -> Result<(), TokenBurnError> {
    if value.map(|value| value.trim().is_empty()).unwrap_or(false) {
        return Err(TokenBurnError::InvalidInput(format!(
            "{} cannot be empty when provided",
            field
        )));
    }
    Ok(())
}
