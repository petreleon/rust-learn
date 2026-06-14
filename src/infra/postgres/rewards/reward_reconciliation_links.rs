use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::reconcile_candidate::RewardReconciliationError;
use crate::db::schema::{transactions_external_transactions, transactions_internal_transactions};
use crate::infra::postgres::rewards::reward_reconciliation_mappers::map_diesel_error;
use crate::models::transaction::{
    NewTransactionExternalTransactionLink, NewTransactionInternalTransactionLink,
};

pub(super) async fn ensure_external_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    external_transaction_id: i64,
) -> Result<bool, RewardReconciliationError> {
    let inserted = diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .on_conflict((
            transactions_external_transactions::transaction_id,
            transactions_external_transactions::external_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await
        .map_err(map_diesel_error)?;
    Ok(inserted > 0)
}

pub(super) async fn ensure_internal_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    internal_transaction_id: i64,
) -> Result<bool, RewardReconciliationError> {
    let inserted = diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .on_conflict((
            transactions_internal_transactions::transaction_id,
            transactions_internal_transactions::internal_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await
        .map_err(map_diesel_error)?;
    Ok(inserted > 0)
}
