use bigdecimal::BigDecimal;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::record_compensation::RewardCompensationError;
use crate::db::schema::{internal_transactions, transactions, transactions_internal_transactions};
use crate::domain::rewards::compensation::REWARD_TRANSACTION_TYPE_COMPENSATION;
use crate::infra::postgres::rewards::reward_compensation_mappers::map_reward_compensation_error;
use crate::models::transaction::{
    NewInternalTransaction, NewTransaction, NewTransactionInternalTransactionLink,
};

pub(super) async fn create_internal_transaction(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> Result<i64, RewardCompensationError> {
    diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .map_err(map_reward_compensation_error)
}

pub(super) async fn create_compensation_transaction(
    conn: &mut AsyncPgConnection,
    internal_transaction_id: i64,
) -> Result<i64, RewardCompensationError> {
    let transaction_id = diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: REWARD_TRANSACTION_TYPE_COMPENSATION,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
        .map_err(map_reward_compensation_error)?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await
        .map_err(map_reward_compensation_error)?;

    Ok(transaction_id)
}
