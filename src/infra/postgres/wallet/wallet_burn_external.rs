use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::burn_tokens::{TokenBurnDraft, TokenBurnError};
use crate::infra::postgres::models::transaction::{
    NewExternalTransaction, NewTransactionExternalTransactionLink,
};
use crate::infra::postgres::schema::{external_transactions, transactions_external_transactions};

const BURN_EXTERNAL_EVENT_TYPE: &str = "burn";

pub(super) async fn maybe_create_burn_external_transaction(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    draft: &TokenBurnDraft,
) -> Result<Option<i64>, TokenBurnError> {
    if draft.transaction_hash.is_none() {
        return Ok(None);
    }
    let Some(address) = draft
        .ethereum_address
        .as_deref()
        .or(draft.platform_address.as_deref())
    else {
        return Ok(None);
    };

    create_burn_external_transaction(
        conn,
        transaction_id,
        BurnExternalTransactionEvidence {
            amount: draft.amount.clone(),
            address,
            chain_id: draft.chain_id,
            contract_address: draft.contract_address.as_deref(),
            transaction_hash: draft.transaction_hash.as_deref(),
            log_index: draft.log_index,
        },
    )
    .await
    .map(Some)
}

pub(super) async fn create_burn_external_transaction(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    evidence: BurnExternalTransactionEvidence<'_>,
) -> Result<i64, TokenBurnError> {
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: evidence.amount,
            blockchain_address: evidence.address,
            chain_id: evidence.chain_id,
            contract_address: evidence.contract_address,
            transaction_hash: evidence.transaction_hash,
            log_index: evidence.log_index,
            event_type: Some(BURN_EXTERNAL_EVENT_TYPE),
            from_address: Some(evidence.address),
            to_address: None,
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))?;

    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))?;

    Ok(external_transaction_id)
}

pub(super) struct BurnExternalTransactionEvidence<'a> {
    pub amount: bigdecimal::BigDecimal,
    pub address: &'a str,
    pub chain_id: Option<i64>,
    pub contract_address: Option<&'a str>,
    pub transaction_hash: Option<&'a str>,
    pub log_index: Option<i64>,
}
