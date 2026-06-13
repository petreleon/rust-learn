use crate::application::wallet::audit_wallet::{WalletAuditError, WalletExternalTransactionAudit};
use crate::infra::postgres::wallet::wallet_audit_external_reward_transactions::load_reward_external_transactions;
use crate::infra::postgres::wallet::wallet_audit_external_wallet_transactions::load_wallet_external_transactions;
use diesel_async::AsyncPgConnection;

pub(super) async fn load_external_transactions(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
    wallet_transaction_ids: &[i64],
) -> Result<Vec<WalletExternalTransactionAudit>, WalletAuditError> {
    let mut audits = Vec::new();
    let mut seen = std::collections::HashSet::new();

    if !candidate_ids.is_empty() {
        load_reward_external_transactions(conn, candidate_ids, &mut audits, &mut seen).await?;
    }

    if !wallet_transaction_ids.is_empty() {
        load_wallet_external_transactions(conn, wallet_transaction_ids, &mut audits, &mut seen)
            .await?;
    }

    audits.sort_by(|left, right| {
        right
            .external_transaction_id
            .cmp(&left.external_transaction_id)
            .then_with(|| right.transaction_id.cmp(&left.transaction_id))
    });

    Ok(audits)
}
