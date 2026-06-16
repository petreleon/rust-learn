use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::burn_tokens::{TokenBurnError, TokenBurnView};
use crate::domain::wallet::burn::{
    TOKEN_BURN_SOURCE_CENTRALIZED, TOKEN_BURN_SOURCE_DECENTRALIZED_DIRECT,
    TOKEN_BURN_SOURCE_PLATFORM_MEDIATED, TOKEN_BURN_STATUS_FAILED,
    TOKEN_BURN_STATUS_LEADERBOARD_INDEXED, TOKEN_BURN_STATUS_NEEDS_RECONCILIATION,
};
use crate::infra::postgres::models::token_burn_request::TokenBurnRequest;
use crate::infra::postgres::schema::token_burn_requests;
use crate::infra::postgres::wallet::wallet_burn_mappers::token_burn_view;

pub(super) async fn finalize_indexed(
    conn: &mut AsyncPgConnection,
    record: TokenBurnRequest,
    now: DateTime<Utc>,
) -> Result<TokenBurnView, TokenBurnError> {
    if record.leaderboard_visible && record.leaderboard_indexed_at.is_none() {
        super::wallet_burn_leaderboard::insert_leaderboard_event(conn, &record, now).await?;
    }
    let ledger_recorded_at = if record.internal_transaction_id.is_some() {
        Some(record.ledger_recorded_at.unwrap_or(now))
    } else {
        record.ledger_recorded_at
    };
    update_record(
        conn,
        record.id,
        IndexedUpdate {
            external_transaction_id: record.external_transaction_id,
            ledger_recorded_at,
            now,
        },
    )
    .await
}

pub(super) async fn mark_failed(
    conn: &mut AsyncPgConnection,
    burn_request_id: i64,
    message: Option<String>,
    now: DateTime<Utc>,
) -> Result<TokenBurnView, TokenBurnError> {
    update_status(
        conn,
        burn_request_id,
        TOKEN_BURN_STATUS_FAILED,
        message,
        now,
    )
    .await
}

pub(super) async fn mark_needs_reconciliation(
    conn: &mut AsyncPgConnection,
    burn_request_id: i64,
    message: String,
    now: DateTime<Utc>,
) -> Result<TokenBurnView, TokenBurnError> {
    update_status(
        conn,
        burn_request_id,
        TOKEN_BURN_STATUS_NEEDS_RECONCILIATION,
        Some(message),
        now,
    )
    .await
}

pub(super) fn missing_reconciliation_reason(record: &TokenBurnRequest) -> Option<String> {
    match record.source.as_str() {
        TOKEN_BURN_SOURCE_CENTRALIZED if record.internal_transaction_id.is_none() => {
            Some("centralized burn is missing internal ledger debit".to_string())
        }
        TOKEN_BURN_SOURCE_DECENTRALIZED_DIRECT | TOKEN_BURN_SOURCE_PLATFORM_MEDIATED
            if record.external_transaction_id.is_none() =>
        {
            Some("burn is missing external transaction evidence".to_string())
        }
        _ => None,
    }
}

async fn update_record(
    conn: &mut AsyncPgConnection,
    burn_request_id: i64,
    update: IndexedUpdate,
) -> Result<TokenBurnView, TokenBurnError> {
    diesel::update(token_burn_requests::table.find(burn_request_id))
        .set((
            token_burn_requests::status.eq(TOKEN_BURN_STATUS_LEADERBOARD_INDEXED),
            token_burn_requests::external_transaction_id.eq(update.external_transaction_id),
            token_burn_requests::last_error.eq(None::<String>),
            token_burn_requests::confirmed_at.eq(Some(update.now)),
            token_burn_requests::ledger_recorded_at.eq(update.ledger_recorded_at),
            token_burn_requests::leaderboard_indexed_at.eq(Some(update.now)),
            token_burn_requests::updated_at.eq(update.now),
        ))
        .get_result::<TokenBurnRequest>(conn)
        .await
        .map(token_burn_view)
        .map_err(|error| TokenBurnError::BurnReconcile(error.to_string()))
}

async fn update_status(
    conn: &mut AsyncPgConnection,
    burn_request_id: i64,
    status: &'static str,
    message: Option<String>,
    now: DateTime<Utc>,
) -> Result<TokenBurnView, TokenBurnError> {
    diesel::update(token_burn_requests::table.find(burn_request_id))
        .set((
            token_burn_requests::status.eq(status),
            token_burn_requests::last_error.eq(message),
            token_burn_requests::updated_at.eq(now),
        ))
        .get_result::<TokenBurnRequest>(conn)
        .await
        .map(token_burn_view)
        .map_err(|error| TokenBurnError::BurnReconcile(error.to_string()))
}

struct IndexedUpdate {
    external_transaction_id: Option<i64>,
    ledger_recorded_at: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
}
