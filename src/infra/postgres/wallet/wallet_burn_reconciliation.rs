use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::burn_tokens::{
    TokenBurnError, TokenBurnReconciliationCommand, TokenBurnView,
};
use crate::domain::wallet::burn::{
    TOKEN_BURN_STATUS_DEPOSIT_PENDING, TOKEN_BURN_STATUS_FAILED,
    TOKEN_BURN_STATUS_NEEDS_RECONCILIATION,
};
use crate::infra::postgres::models::token_burn_request::TokenBurnRequest;
use crate::infra::postgres::schema::token_burn_requests;
use crate::infra::postgres::wallet::wallet_burn_external::{
    create_burn_external_transaction, BurnExternalTransactionEvidence,
};
use crate::infra::postgres::wallet::wallet_burn_mappers::token_burn_view;
use crate::infra::postgres::wallet::wallet_burn_reconciliation_state::{
    finalize_indexed, mark_failed, mark_needs_reconciliation, missing_reconciliation_reason,
};

pub(super) async fn list_reconciliation_burns(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<TokenBurnView>, TokenBurnError> {
    token_burn_requests::table
        .filter(
            token_burn_requests::status
                .eq_any([
                    TOKEN_BURN_STATUS_DEPOSIT_PENDING,
                    TOKEN_BURN_STATUS_NEEDS_RECONCILIATION,
                ])
                .or(token_burn_requests::leaderboard_visible
                    .eq(true)
                    .and(token_burn_requests::leaderboard_indexed_at.is_null())),
        )
        .order(token_burn_requests::updated_at.desc())
        .limit(100)
        .load::<TokenBurnRequest>(conn)
        .await
        .map(|rows| rows.into_iter().map(token_burn_view).collect())
        .map_err(|error| TokenBurnError::BurnLoad(error.to_string()))
}

pub(super) async fn list_failed_burns(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<TokenBurnView>, TokenBurnError> {
    token_burn_requests::table
        .filter(token_burn_requests::status.eq(TOKEN_BURN_STATUS_FAILED))
        .order(token_burn_requests::updated_at.desc())
        .limit(100)
        .load::<TokenBurnRequest>(conn)
        .await
        .map(|rows| rows.into_iter().map(token_burn_view).collect())
        .map_err(|error| TokenBurnError::BurnLoad(error.to_string()))
}

pub(super) async fn reconcile_burn_request(
    conn: &mut AsyncPgConnection,
    burn_request_id: i64,
    command: TokenBurnReconciliationCommand,
) -> Result<TokenBurnView, TokenBurnError> {
    conn.transaction::<_, TokenBurnError, _>(|conn| {
        Box::pin(async move {
            let now = Utc::now();
            let mut record = find_burn(conn, burn_request_id).await?;
            if command.mark_failed.unwrap_or(false) {
                return mark_failed(conn, record.id, command.error_message, now).await;
            }
            maybe_attach_external_evidence(conn, &mut record, &command).await?;
            match missing_reconciliation_reason(&record) {
                Some(reason) => mark_needs_reconciliation(conn, record.id, reason, now).await,
                None => finalize_indexed(conn, record, now).await,
            }
        })
    })
    .await
}

async fn find_burn(
    conn: &mut AsyncPgConnection,
    burn_request_id: i64,
) -> Result<TokenBurnRequest, TokenBurnError> {
    token_burn_requests::table
        .find(burn_request_id)
        .first::<TokenBurnRequest>(conn)
        .await
        .map_err(|error| match error {
            diesel::result::Error::NotFound => TokenBurnError::BurnNotFound,
            other => TokenBurnError::BurnLoad(other.to_string()),
        })
}

async fn maybe_attach_external_evidence(
    conn: &mut AsyncPgConnection,
    record: &mut TokenBurnRequest,
    command: &TokenBurnReconciliationCommand,
) -> Result<(), TokenBurnError> {
    if record.external_transaction_id.is_some() || command.transaction_hash.is_none() {
        return Ok(());
    }
    let Some(transaction_id) = record.transaction_id else {
        return Ok(());
    };
    let address = normalized(
        command
            .ethereum_address
            .as_ref()
            .or(command.platform_address.as_ref()),
    );
    let contract_address = command.contract_address.as_deref().map(normalize);
    let transaction_hash = command.transaction_hash.as_deref().map(normalize);
    let external_transaction_id = create_burn_external_transaction(
        conn,
        transaction_id,
        BurnExternalTransactionEvidence {
            amount: record.amount.clone(),
            address: &address,
            chain_id: command.chain_id,
            contract_address: contract_address.as_deref(),
            transaction_hash: transaction_hash.as_deref(),
            log_index: command.log_index,
        },
    )
    .await?;
    record.external_transaction_id = Some(external_transaction_id);
    Ok(())
}

fn normalized(value: Option<&String>) -> String {
    value.map(|value| normalize(value)).unwrap_or_default()
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
