use bigdecimal::BigDecimal;

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxAuditEventView, WalletTokenTaxError, WalletTokenTaxOperation,
    WalletTokenTaxSettings, WalletTokenTaxStore, WalletTokenTaxView,
};

pub async fn list_token_taxes(
    store: &mut impl WalletTokenTaxStore,
    actor_user_id: i32,
) -> Result<WalletTokenTaxSettings, WalletTokenTaxError> {
    ensure_can_view_token_tax_configuration(store, actor_user_id).await?;
    let deposit = load_token_tax(store, WalletTokenTaxOperation::Deposit).await?;
    let retire = load_token_tax(store, WalletTokenTaxOperation::Retire).await?;

    Ok(WalletTokenTaxSettings { deposit, retire })
}

pub async fn list_token_tax_audit(
    store: &mut impl WalletTokenTaxStore,
    actor_user_id: i32,
) -> Result<Vec<WalletTokenTaxAuditEventView>, WalletTokenTaxError> {
    ensure_can_view_token_tax_configuration(store, actor_user_id).await?;
    store.list_token_tax_audit().await
}

pub async fn set_token_tax(
    store: &mut impl WalletTokenTaxStore,
    actor_user_id: i32,
    operation: WalletTokenTaxOperation,
    amount: BigDecimal,
) -> Result<WalletTokenTaxView, WalletTokenTaxError> {
    if !store.can_set_token_tax(actor_user_id, operation).await? {
        return Err(WalletTokenTaxError::PermissionDenied);
    }

    validate_non_negative_amount(&amount, "tax_amount")?;
    let previous_amount = store.load_token_tax(operation).await?;
    validate_non_negative_amount(&previous_amount, "stored tax amount")?;
    store
        .save_token_tax(actor_user_id, operation, previous_amount, amount.clone())
        .await?;
    Ok(token_tax_view(operation, amount))
}

async fn load_token_tax(
    store: &mut impl WalletTokenTaxStore,
    operation: WalletTokenTaxOperation,
) -> Result<WalletTokenTaxView, WalletTokenTaxError> {
    let amount = store.load_token_tax(operation).await?;
    validate_non_negative_amount(&amount, "stored tax amount")?;
    Ok(token_tax_view(operation, amount))
}

fn validate_non_negative_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletTokenTaxError> {
    if amount < &BigDecimal::from(0) {
        return Err(WalletTokenTaxError::InvalidInput(format!(
            "{} cannot be negative",
            field_name
        )));
    }

    Ok(())
}

async fn ensure_can_view_token_tax_configuration(
    store: &mut impl WalletTokenTaxStore,
    actor_user_id: i32,
) -> Result<(), WalletTokenTaxError> {
    if store
        .can_view_token_tax_configuration(actor_user_id)
        .await?
    {
        Ok(())
    } else {
        Err(WalletTokenTaxError::PermissionDenied)
    }
}

fn token_tax_view(operation: WalletTokenTaxOperation, amount: BigDecimal) -> WalletTokenTaxView {
    WalletTokenTaxView {
        operation: operation.as_str(),
        tax_amount: amount.to_string(),
    }
}
