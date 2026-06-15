use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletExternalTransactionAudit};
use crate::db::schema::{external_transactions, transactions_external_transactions};
use crate::infra::postgres::wallet::wallet_audit_external_rows::{
    wallet_external_transaction_audit, WalletExternalTransactionRow,
};
use crate::infra::postgres::wallet::wallet_audit_mappers::map_wallet_audit_error;

pub(super) async fn load_wallet_external_transactions(
    conn: &mut AsyncPgConnection,
    wallet_transaction_ids: &[i64],
    audits: &mut Vec<WalletExternalTransactionAudit>,
    seen: &mut std::collections::HashSet<(i64, i64)>,
) -> Result<(), WalletAuditError> {
    let rows = transactions_external_transactions::table
        .inner_join(
            external_transactions::table
                .on(transactions_external_transactions::external_transaction_id
                    .eq(external_transactions::id)),
        )
        .filter(transactions_external_transactions::transaction_id.eq_any(wallet_transaction_ids))
        .select((
            transactions_external_transactions::transaction_id,
            external_transactions::id,
            external_transactions::amount,
            external_transactions::blockchain_address,
            external_transactions::chain_id,
            external_transactions::contract_address,
            external_transactions::transaction_hash,
            external_transactions::log_index,
            external_transactions::event_type,
            external_transactions::from_address,
            external_transactions::to_address,
        ))
        .order(external_transactions::id.desc())
        .load::<WalletExternalTransactionRow>(conn)
        .await
        .map_err(map_wallet_audit_error)?;

    for row in rows {
        let audit = wallet_external_transaction_audit(row)?;
        if seen.insert((audit.transaction_id, audit.external_transaction_id)) {
            audits.push(audit);
        }
    }

    Ok(())
}
