use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::burn_tokens::{TokenBurnDraft, TokenBurnError, TokenBurnView};
use crate::domain::wallet::burn::{TokenBurnSource, TOKEN_BURN_STATUS_LEADERBOARD_INDEXED};
use crate::infra::postgres::models::token_burn_fee_record::NewTokenBurnFeeRecord;
use crate::infra::postgres::models::token_burn_request::{NewTokenBurnRequest, TokenBurnRequest};
use crate::infra::postgres::models::transaction::NewTransaction;
use crate::infra::postgres::schema::{token_burn_fee_records, token_burn_requests, transactions};
use crate::infra::postgres::wallet::wallet_burn_builders::new_burn_request;
use crate::infra::postgres::wallet::wallet_burn_external::maybe_create_burn_external_transaction;
use crate::infra::postgres::wallet::wallet_burn_ledger::debit_burn_amount;
use crate::infra::postgres::wallet::wallet_burn_mappers::token_burn_view;
use crate::infra::postgres::wallet::wallet_link_records::{
    link_organization_wallet_record, link_user_wallet_record,
};

const BURN_TRANSACTION_TYPE: &str = "token_burn";

pub(super) async fn create_burn_request(
    conn: &mut AsyncPgConnection,
    draft: TokenBurnDraft,
) -> Result<TokenBurnView, TokenBurnError> {
    if let Some(existing) = find_burn_by_key(conn, &draft.idempotency_key).await? {
        return Ok(token_burn_view(existing));
    }

    conn.transaction::<_, TokenBurnError, _>(|conn| {
        Box::pin(async move {
            let now = Utc::now();
            let wallet_id = burn_wallet_id(conn, &draft).await?;
            let transaction_id = create_burn_transaction(conn).await?;
            let external_transaction_id =
                maybe_create_burn_external_transaction(conn, transaction_id, &draft).await?;
            let internal_transaction_id = match draft.source {
                TokenBurnSource::CentralizedWallet => Some(
                    debit_burn_amount(conn, wallet_id, transaction_id, draft.amount.clone())
                        .await?,
                ),
                _ => None,
            };
            let record = insert_burn_request(
                conn,
                new_burn_request(
                    draft,
                    wallet_id,
                    transaction_id,
                    external_transaction_id,
                    internal_transaction_id,
                    now,
                ),
            )
            .await?;
            insert_fee_record(conn, &record).await?;
            if record.status == TOKEN_BURN_STATUS_LEADERBOARD_INDEXED {
                super::wallet_burn_leaderboard::insert_leaderboard_event(conn, &record, now)
                    .await?;
            }
            Ok(token_burn_view(record))
        })
    })
    .await
}

pub(super) async fn list_user_burns(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Vec<TokenBurnView>, TokenBurnError> {
    token_burn_requests::table
        .filter(token_burn_requests::user_id.eq(user_id))
        .order(token_burn_requests::created_at.desc())
        .limit(100)
        .load::<TokenBurnRequest>(conn)
        .await
        .map(|rows| rows.into_iter().map(token_burn_view).collect())
        .map_err(|error| TokenBurnError::BurnLoad(error.to_string()))
}

pub(super) async fn list_organization_burns(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<TokenBurnView>, TokenBurnError> {
    token_burn_requests::table
        .filter(token_burn_requests::organization_id.eq(organization_id))
        .order(token_burn_requests::created_at.desc())
        .limit(100)
        .load::<TokenBurnRequest>(conn)
        .await
        .map(|rows| rows.into_iter().map(token_burn_view).collect())
        .map_err(|error| TokenBurnError::BurnLoad(error.to_string()))
}

async fn find_burn_by_key(
    conn: &mut AsyncPgConnection,
    idempotency_key: &str,
) -> Result<Option<TokenBurnRequest>, TokenBurnError> {
    token_burn_requests::table
        .filter(token_burn_requests::idempotency_key.eq(idempotency_key))
        .first::<TokenBurnRequest>(conn)
        .await
        .optional()
        .map_err(|error| TokenBurnError::BurnLoad(error.to_string()))
}

async fn burn_wallet_id(
    conn: &mut AsyncPgConnection,
    draft: &TokenBurnDraft,
) -> Result<i32, TokenBurnError> {
    match (draft.user_id, draft.organization_id) {
        (Some(user_id), None) => link_user_wallet_record(conn, user_id)
            .await
            .map(|linked| linked.wallet.id)
            .map_err(map_wallet_link_error),
        (None, Some(organization_id)) => link_organization_wallet_record(conn, organization_id)
            .await
            .map(|linked| linked.wallet.id)
            .map_err(map_wallet_link_error),
        _ => Err(TokenBurnError::BurnCreate(
            "invalid burn attribution".to_string(),
        )),
    }
}

async fn create_burn_transaction(conn: &mut AsyncPgConnection) -> Result<i64, TokenBurnError> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: BURN_TRANSACTION_TYPE,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))
}

async fn insert_burn_request(
    conn: &mut AsyncPgConnection,
    request: NewTokenBurnRequest,
) -> Result<TokenBurnRequest, TokenBurnError> {
    diesel::insert_into(token_burn_requests::table)
        .values(request)
        .get_result(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))
}

async fn insert_fee_record(
    conn: &mut AsyncPgConnection,
    record: &TokenBurnRequest,
) -> Result<(), TokenBurnError> {
    diesel::insert_into(token_burn_fee_records::table)
        .values(NewTokenBurnFeeRecord {
            burn_request_id: record.id,
            actor_user_id: record.actor_user_id,
            fee_path: record.fee_path.clone(),
            amount: record.fee_amount.clone(),
            transaction_id: record.transaction_id,
            deposit_intent_id: record.deposit_intent_id,
        })
        .execute(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))?;
    Ok(())
}

fn map_wallet_link_error(
    error: crate::application::wallet::link_wallet::WalletLinkError,
) -> TokenBurnError {
    match error {
        crate::application::wallet::link_wallet::WalletLinkError::WalletCreate(message) => {
            TokenBurnError::WalletCreate(message)
        }
        crate::application::wallet::link_wallet::WalletLinkError::WalletLoad(message) => {
            TokenBurnError::WalletLoad(message)
        }
        other => TokenBurnError::WalletLoad(format!("{:?}", other)),
    }
}
