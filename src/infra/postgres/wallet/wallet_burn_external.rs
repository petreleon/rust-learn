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

    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: draft.amount.clone(),
            blockchain_address: address,
            chain_id: draft.chain_id,
            contract_address: draft.contract_address.as_deref(),
            transaction_hash: draft.transaction_hash.as_deref(),
            log_index: draft.log_index,
            event_type: Some(BURN_EXTERNAL_EVENT_TYPE),
            from_address: Some(address),
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

    Ok(Some(external_transaction_id))
}
