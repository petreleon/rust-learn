use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::retire_tokens::WalletRetirementError;
use crate::db::schema::{internal_transactions, transactions_internal_transactions, wallets};
use crate::infra::postgres::models::transaction::{
    NewInternalTransaction, NewTransactionInternalTransactionLink,
};

pub(super) async fn apply_retirement_ledger_entries(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    transaction_id: i64,
    amount: BigDecimal,
    tax_amount: BigDecimal,
) -> Result<Vec<i64>, WalletRetirementError> {
    let mut internal_transaction_ids = Vec::new();
    internal_transaction_ids
        .push(apply_retirement_ledger_entry(conn, wallet_id, transaction_id, -amount).await?);

    if tax_amount > BigDecimal::from(0) {
        internal_transaction_ids.push(
            apply_retirement_ledger_entry(conn, wallet_id, transaction_id, -tax_amount).await?,
        );
    }

    Ok(internal_transaction_ids)
}

async fn apply_retirement_ledger_entry(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    transaction_id: i64,
    amount: BigDecimal,
) -> Result<i64, WalletRetirementError> {
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
    .map_err(WalletRetirementError::from)?;

    if updated == 0 {
        return Err(WalletRetirementError::InsufficientFunds);
    }

    let internal_transaction_id = diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .map_err(WalletRetirementError::from)?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await
        .map_err(WalletRetirementError::from)?;

    Ok(internal_transaction_id)
}
