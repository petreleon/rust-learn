use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletInternalTransactionAudit};
use crate::db::schema::{internal_transactions, transactions, transactions_internal_transactions};
use crate::infra::postgres::wallet::wallet_audit_mappers::map_wallet_audit_error;

pub(super) async fn load_internal_transactions(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> Result<Vec<WalletInternalTransactionAudit>, WalletAuditError> {
    let rows = internal_transactions::table
        .inner_join(
            transactions_internal_transactions::table.on(internal_transactions::id
                .eq(transactions_internal_transactions::internal_transaction_id)),
        )
        .inner_join(
            transactions::table
                .on(transactions_internal_transactions::transaction_id.eq(transactions::id)),
        )
        .filter(internal_transactions::wallet_id.eq(wallet_id))
        .select((
            internal_transactions::id,
            transactions::id,
            transactions::type_,
            internal_transactions::amount,
            transactions::created_at,
        ))
        .order(transactions::created_at.desc())
        .load::<(i64, i64, String, bigdecimal::BigDecimal, DateTime<Utc>)>(conn)
        .await
        .map_err(map_wallet_audit_error)?;

    Ok(rows
        .into_iter()
        .map(
            |(internal_transaction_id, transaction_id, transaction_type, amount, created_at)| {
                WalletInternalTransactionAudit {
                    internal_transaction_id,
                    transaction_id,
                    transaction_type,
                    amount: amount.to_string(),
                    created_at,
                }
            },
        )
        .collect())
}
