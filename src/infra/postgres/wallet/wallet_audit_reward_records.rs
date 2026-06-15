use std::collections::HashMap;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletRewardRecordAudit};
use crate::db::schema::{reward_candidates, reward_payout_records, reward_wallet_credit_records};
use crate::domain::rewards::candidate::reconciliation::{
    reward_reconciliation_status, RewardReconciliationFacts,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::models::reward_payout_record::RewardPayoutRecord;
use crate::infra::postgres::models::reward_wallet_credit_record::RewardWalletCreditRecord;
use crate::infra::postgres::wallet::wallet_audit_mappers::map_wallet_audit_error;

pub(super) async fn load_reward_records(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
) -> Result<Vec<WalletRewardRecordAudit>, WalletAuditError> {
    if candidate_ids.is_empty() {
        return Ok(Vec::new());
    }

    let candidates = reward_candidates::table
        .filter(reward_candidates::id.eq_any(candidate_ids))
        .order(reward_candidates::created_at.desc())
        .load::<RewardCandidate>(conn)
        .await
        .map_err(map_wallet_audit_error)?;

    let credit_records = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq_any(candidate_ids))
        .load::<RewardWalletCreditRecord>(conn)
        .await
        .map_err(map_wallet_audit_error)?
        .into_iter()
        .map(|record| (record.reward_candidate_id, record))
        .collect::<HashMap<_, _>>();

    let payout_records = reward_payout_records::table
        .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
        .load::<RewardPayoutRecord>(conn)
        .await
        .map_err(map_wallet_audit_error)?
        .into_iter()
        .map(|record| (record.reward_candidate_id, record))
        .collect::<HashMap<_, _>>();

    candidates
        .into_iter()
        .map(|candidate| {
            let candidate_status = RewardCandidateStatus::parse(&candidate.status)
                .map_err(|error| WalletAuditError::AuditLoad(error.to_string()))?;
            let credit_record = credit_records.get(&candidate.id);
            let payout_record = payout_records.get(&candidate.id);
            Ok(WalletRewardRecordAudit {
                reward_candidate_id: candidate.id,
                candidate_status,
                reconciliation_status: reward_reconciliation_status(RewardReconciliationFacts {
                    candidate_status: candidate_status.as_str(),
                    has_credit_record: credit_record.is_some(),
                    has_notification_record: credit_record
                        .and_then(|record| record.notification_id)
                        .is_some(),
                    has_payout_record: payout_record.is_some(),
                }),
                approved_amount: candidate.approved_amount.as_ref().map(ToString::to_string),
                wallet_credit_record_id: credit_record.map(|record| record.id),
                wallet_credit_transaction_id: credit_record.map(|record| record.transaction_id),
                internal_transaction_id: credit_record.map(|record| record.internal_transaction_id),
                payout_record_id: payout_record.map(|record| record.id),
                payout_transaction_id: payout_record.map(|record| record.transaction_id),
                external_transaction_id: payout_record.map(|record| record.external_transaction_id),
                notification_id: credit_record.and_then(|record| record.notification_id),
                notified_at: credit_record.and_then(|record| record.notified_at),
                created_at: candidate.created_at,
                updated_at: candidate.updated_at,
            })
        })
        .collect()
}
