use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::db::schema::{
    external_transactions, internal_transactions, transactions, transactions_external_transactions,
    transactions_internal_transactions, wallets,
};
use crate::infra::postgres::models::transaction::{
    ExternalTransaction, NewExternalTransaction, NewInternalTransaction, NewTransaction,
    NewTransactionExternalTransactionLink, NewTransactionInternalTransactionLink,
};
use crate::infra::postgres::models::wallet::NewWallet;

pub(in crate::infra::postgres::wallet) fn find_user_wallet_id(
    user_id: i32,
    conn: &mut PgConnection,
) -> QueryResult<Option<i32>> {
    wallets::table
        .select(wallets::id)
        .filter(
            wallets::user_id
                .eq(user_id)
                .and(wallets::organization_id.is_null()),
        )
        .first(conn)
        .optional()
}

pub(in crate::infra::postgres::wallet) fn find_organization_wallet_id(
    organization_id: i32,
    conn: &mut PgConnection,
) -> QueryResult<Option<i32>> {
    wallets::table
        .select(wallets::id)
        .filter(
            wallets::organization_id
                .eq(organization_id)
                .and(wallets::user_id.is_null()),
        )
        .first(conn)
        .optional()
}

pub(in crate::infra::postgres::wallet) fn create_wallet(
    new_wallet: NewWallet,
    conn: &mut PgConnection,
) -> QueryResult<i32> {
    diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .returning(wallets::id)
        .get_result(conn)
}

pub(in crate::infra::postgres::wallet) fn update_wallet_balance_guarded(
    wallet_id: i32,
    amount: BigDecimal,
    conn: &mut PgConnection,
) -> QueryResult<usize> {
    diesel::update(
        wallets::table.filter(
            wallets::id
                .eq(wallet_id)
                .and((wallets::value + amount.clone()).ge(BigDecimal::from(0))),
        ),
    )
    .set(wallets::value.eq(wallets::value + amount))
    .execute(conn)
}

pub(in crate::infra::postgres::wallet) fn lock_wallets_for_update(
    ids: Vec<i32>,
    conn: &mut PgConnection,
) -> QueryResult<Vec<(i32, BigDecimal)>> {
    wallets::table
        .select((wallets::id, wallets::value))
        .filter(wallets::id.eq_any(ids))
        .order(wallets::id.asc())
        .for_update()
        .load(conn)
}

pub(in crate::infra::postgres::wallet) fn create_transaction(
    type_: &str,
    conn: &mut PgConnection,
) -> QueryResult<i64> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction { type_ })
        .returning(transactions::id)
        .get_result(conn)
}

pub(in crate::infra::postgres::wallet) fn create_internal_transaction(
    wallet_id: i32,
    amount: BigDecimal,
    conn: &mut PgConnection,
) -> QueryResult<i64> {
    diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
}

pub(in crate::infra::postgres::wallet) fn link_internal_transaction(
    transaction_id: i64,
    internal_transaction_id: i64,
    conn: &mut PgConnection,
) -> QueryResult<usize> {
    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
}

pub(in crate::infra::postgres::wallet) fn find_external_transaction_by_chain_tx_log(
    chain_id: i64,
    transaction_hash: &str,
    log_index: i64,
    conn: &mut PgConnection,
) -> QueryResult<Option<ExternalTransaction>> {
    external_transactions::table
        .filter(external_transactions::chain_id.eq(chain_id))
        .filter(external_transactions::transaction_hash.eq(transaction_hash))
        .filter(external_transactions::log_index.eq(log_index))
        .first(conn)
        .optional()
}

pub(in crate::infra::postgres::wallet) fn create_external_transaction(
    new_external_transaction: NewExternalTransaction<'_>,
    conn: &mut PgConnection,
) -> QueryResult<i64> {
    diesel::insert_into(external_transactions::table)
        .values(&new_external_transaction)
        .returning(external_transactions::id)
        .get_result(conn)
}

pub(in crate::infra::postgres::wallet) fn link_external_transaction(
    transaction_id: i64,
    external_transaction_id: i64,
    conn: &mut PgConnection,
) -> QueryResult<usize> {
    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
}

pub(in crate::infra::postgres::wallet) fn find_transaction_for_external(
    external_transaction_id: i64,
    conn: &mut PgConnection,
) -> QueryResult<Option<i64>> {
    transactions_external_transactions::table
        .filter(
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        )
        .select(transactions_external_transactions::transaction_id)
        .first(conn)
        .optional()
}
