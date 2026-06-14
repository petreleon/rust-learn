use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletExternalTransactionAudit};
use crate::db::schema::{external_transactions, reward_payout_records};
use crate::infra::postgres::wallet::wallet_audit_external_rows::{
    reward_external_transaction_audit, RewardExternalTransactionRow,
};
use crate::infra::postgres::wallet::wallet_audit_mappers::map_wallet_audit_error;

pub(super) async fn load_reward_external_transactions(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
    audits: &mut Vec<WalletExternalTransactionAudit>,
    seen: &mut std::collections::HashSet<(i64, i64)>,
) -> Result<(), WalletAuditError> {
    let rows = reward_payout_records::table
        .inner_join(
            external_transactions::table
                .on(reward_payout_records::external_transaction_id.eq(external_transactions::id)),
        )
        .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
        .select((
            reward_payout_records::reward_candidate_id,
            reward_payout_records::transaction_id,
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
        .load::<RewardExternalTransactionRow>(conn)
        .await
        .map_err(map_wallet_audit_error)?;

    for row in rows {
        let audit = reward_external_transaction_audit(row);
        seen.insert((audit.transaction_id, audit.external_transaction_id));
        audits.push(audit);
    }

    Ok(())
}
