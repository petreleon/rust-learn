use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletDepositIntentAudit};
use crate::domain::rewards::token::RewardTokenEventType;
use crate::domain::wallet::deposit::WalletDepositStatus;
use crate::infra::postgres::schema::wallet_token_deposit_intents;
use crate::infra::postgres::wallet::wallet_audit_mappers::map_wallet_audit_error;

type DepositIntentRow = (
    i64,
    i32,
    i32,
    String,
    String,
    bigdecimal::BigDecimal,
    bigdecimal::BigDecimal,
    String,
    String,
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<String>,
    Option<i64>,
    Option<i64>,
    String,
    bool,
    String,
    Option<String>,
    DateTime<Utc>,
    DateTime<Utc>,
    Option<DateTime<Utc>>,
);

pub(super) async fn load_deposit_intents(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> Result<Vec<WalletDepositIntentAudit>, WalletAuditError> {
    let rows = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::wallet_id.eq(wallet_id))
        .select((
            wallet_token_deposit_intents::id,
            wallet_token_deposit_intents::user_id,
            wallet_token_deposit_intents::wallet_id,
            wallet_token_deposit_intents::ethereum_address,
            wallet_token_deposit_intents::platform_address,
            wallet_token_deposit_intents::amount,
            wallet_token_deposit_intents::tax_amount,
            wallet_token_deposit_intents::gas_payer,
            wallet_token_deposit_intents::status,
            wallet_token_deposit_intents::chain_id,
            wallet_token_deposit_intents::contract_address,
            wallet_token_deposit_intents::transaction_hash,
            wallet_token_deposit_intents::log_index,
            wallet_token_deposit_intents::event_type,
            wallet_token_deposit_intents::external_transaction_id,
            wallet_token_deposit_intents::transaction_id,
            wallet_token_deposit_intents::wallet_provider,
            wallet_token_deposit_intents::metamask_required,
            wallet_token_deposit_intents::wallet_action,
            wallet_token_deposit_intents::last_error,
            wallet_token_deposit_intents::created_at,
            wallet_token_deposit_intents::updated_at,
            wallet_token_deposit_intents::credited_at,
        ))
        .order(wallet_token_deposit_intents::created_at.desc())
        .load::<DepositIntentRow>(conn)
        .await
        .map_err(map_wallet_audit_error)?;

    rows.into_iter().map(deposit_intent_audit).collect()
}

fn deposit_intent_audit(
    row: DepositIntentRow,
) -> Result<WalletDepositIntentAudit, WalletAuditError> {
    let (
        id,
        user_id,
        wallet_id,
        ethereum_address,
        platform_address,
        amount,
        tax_amount,
        gas_payer,
        status,
        chain_id,
        contract_address,
        transaction_hash,
        log_index,
        event_type,
        external_transaction_id,
        transaction_id,
        wallet_provider,
        metamask_required,
        wallet_action,
        last_error,
        created_at,
        updated_at,
        credited_at,
    ) = row;

    Ok(WalletDepositIntentAudit {
        id,
        user_id,
        wallet_id,
        ethereum_address,
        platform_address,
        amount: amount.to_string(),
        tax_amount: tax_amount.to_string(),
        gas_payer,
        status: WalletDepositStatus::parse(&status)
            .map_err(|error| WalletAuditError::AuditLoad(error.to_string()))?,
        chain_id,
        contract_address,
        transaction_hash,
        log_index,
        event_type: event_type
            .as_deref()
            .map(RewardTokenEventType::normalize)
            .transpose()
            .map_err(|error| WalletAuditError::AuditLoad(error.to_string()))?,
        external_transaction_id,
        transaction_id,
        wallet_provider,
        metamask_required,
        wallet_action,
        last_error,
        created_at,
        updated_at,
        credited_at,
    })
}
