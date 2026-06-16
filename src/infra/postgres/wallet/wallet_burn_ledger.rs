use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::burn_tokens::TokenBurnError;
use crate::infra::postgres::models::transaction::{
    NewInternalTransaction, NewTransactionInternalTransactionLink,
};
use crate::infra::postgres::schema::{
    internal_transactions, transactions_internal_transactions, wallets,
};

pub(super) async fn debit_burn_amount(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    transaction_id: i64,
    amount: BigDecimal,
) -> Result<i64, TokenBurnError> {
    let updated = diesel::update(
        wallets::table.filter(
            wallets::id
                .eq(wallet_id)
                .and((wallets::value - amount.clone()).ge(BigDecimal::from(0))),
        ),
    )
    .set(wallets::value.eq(wallets::value - amount.clone()))
    .execute(conn)
    .await
    .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))?;

    if updated == 0 {
        return Err(TokenBurnError::InsufficientFunds);
    }

    let internal_transaction_id = diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction {
            wallet_id,
            amount: -amount,
        })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))?;

    Ok(internal_transaction_id)
}
