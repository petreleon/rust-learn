use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexError,
};
use crate::db::schema::{
    external_transactions, internal_transactions, transactions, transactions_external_transactions,
    transactions_internal_transactions, wallets,
};
use crate::domain::wallet::deposit::WALLET_DEPOSIT_TRANSACTION_TYPE;
use crate::infra::postgres::models::transaction::{
    NewExternalTransaction, NewInternalTransaction, NewTransaction,
    NewTransactionExternalTransactionLink, NewTransactionInternalTransactionLink,
};

pub(super) async fn create_deposit_transaction(
    conn: &mut AsyncPgConnection,
) -> Result<i64, WalletDepositIndexError> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: WALLET_DEPOSIT_TRANSACTION_TYPE,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
        .map_err(WalletDepositIndexError::from)
}

pub(super) async fn create_deposit_external_transaction(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    event: &ObservedWalletDepositEvent,
) -> Result<i64, WalletDepositIndexError> {
    let from_address = normalize_address(&event.from_address);
    let to_address = normalize_address(&event.to_address);
    let contract_address = normalize_address(&event.contract_address);
    let transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();

    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: event.amount.clone(),
            blockchain_address: &from_address,
            chain_id: Some(event.chain_id),
            contract_address: Some(&contract_address),
            transaction_hash: Some(&transaction_hash),
            log_index: Some(event.log_index),
            event_type: Some(event.event_type.as_str()),
            from_address: Some(&from_address),
            to_address: Some(&to_address),
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await
        .map_err(WalletDepositIndexError::from)?;

    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await
        .map_err(WalletDepositIndexError::from)?;

    Ok(external_transaction_id)
}

pub(super) async fn apply_deposit_ledger_entries(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    transaction_id: i64,
    amount: BigDecimal,
    tax_amount: BigDecimal,
) -> Result<Vec<i64>, WalletDepositIndexError> {
    let mut ids = Vec::new();
    ids.push(apply_deposit_ledger_entry(conn, wallet_id, transaction_id, amount).await?);
    if tax_amount > BigDecimal::from(0) {
        ids.push(apply_deposit_ledger_entry(conn, wallet_id, transaction_id, -tax_amount).await?);
    }
    Ok(ids)
}

async fn apply_deposit_ledger_entry(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    transaction_id: i64,
    amount: BigDecimal,
) -> Result<i64, WalletDepositIndexError> {
    let updated = diesel::update(
        wallets::table.filter(
            wallets::id
                .eq(wallet_id)
                .and((wallets::value + amount.clone()).ge(BigDecimal::from(0))),
        ),
    )
    .set(wallets::value.eq(wallets::value + amount.clone()))
    .execute(conn)
    .await
    .map_err(WalletDepositIndexError::from)?;

    if updated == 0 {
        return Err(WalletDepositIndexError::InsufficientFunds);
    }

    let internal_transaction_id = diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .map_err(WalletDepositIndexError::from)?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await
        .map_err(WalletDepositIndexError::from)?;

    Ok(internal_transaction_id)
}

pub(super) fn normalize_address(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
