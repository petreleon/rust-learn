use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletCompensationRecordAudit};
use crate::infra::postgres::models::reward_compensation_record::RewardCompensationRecord;
use crate::infra::postgres::schema::reward_compensation_records;
use crate::infra::postgres::wallet::wallet_audit_mappers::map_wallet_audit_error;

pub(super) async fn load_compensation_records(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> Result<Vec<WalletCompensationRecordAudit>, WalletAuditError> {
    let records = reward_compensation_records::table
        .filter(reward_compensation_records::wallet_id.eq(wallet_id))
        .order(reward_compensation_records::created_at.desc())
        .load::<RewardCompensationRecord>(conn)
        .await
        .map_err(map_wallet_audit_error)?;

    Ok(records
        .into_iter()
        .map(|record| WalletCompensationRecordAudit {
            id: record.id,
            reward_candidate_id: record.reward_candidate_id,
            wallet_id: record.wallet_id,
            transaction_id: record.transaction_id,
            internal_transaction_id: record.internal_transaction_id,
            amount: record.amount.to_string(),
            reason: record.reason,
            idempotency_key: record.idempotency_key,
            created_by_user_id: record.created_by_user_id,
            created_at: record.created_at,
        })
        .collect())
}
