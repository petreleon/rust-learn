use std::collections::HashMap;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_reward_dashboard::{
    reconciliation_mismatch_candidate_statuses, reward_reconciliation_mismatch_row,
    PlatformRewardDashboardError, RewardReconciliationMismatchRowFact,
    RewardReconciliationMismatchRowOutput,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::reporting::platform_reward_dashboard_summaries::map_diesel_error;
use crate::infra::postgres::schema::{
    reward_candidates, reward_payout_records, reward_wallet_credit_records,
};

pub(super) async fn reward_reconciliation_mismatches(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<RewardReconciliationMismatchRowOutput>, PlatformRewardDashboardError> {
    let candidates = reward_candidates::table
        .filter(reward_candidates::status.eq_any(reconciliation_statuses()))
        .order(reward_candidates::updated_at.desc())
        .load::<RewardCandidate>(conn)
        .await
        .map_err(map_diesel_error)?;
    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    let candidate_ids = candidates
        .iter()
        .map(|candidate| candidate.id)
        .collect::<Vec<_>>();
    let payout_records = payout_record_map(conn, &candidate_ids).await?;
    let credit_records = credit_record_map(conn, &candidate_ids).await?;

    Ok(candidates
        .into_iter()
        .map(|candidate| mismatch_row(candidate, &payout_records, &credit_records))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .take(50)
        .collect())
}

fn reconciliation_statuses() -> [&'static str; 5] {
    reconciliation_mismatch_candidate_statuses().map(RewardCandidateStatus::as_str)
}

async fn payout_record_map(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
) -> Result<HashMap<i64, bool>, PlatformRewardDashboardError> {
    reward_payout_records::table
        .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
        .select(reward_payout_records::reward_candidate_id)
        .load::<i64>(conn)
        .await
        .map(|ids| {
            ids.into_iter()
                .map(|candidate_id| (candidate_id, true))
                .collect()
        })
        .map_err(map_diesel_error)
}

async fn credit_record_map(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
) -> Result<HashMap<i64, Option<i64>>, PlatformRewardDashboardError> {
    reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq_any(candidate_ids))
        .select((
            reward_wallet_credit_records::reward_candidate_id,
            reward_wallet_credit_records::notification_id,
        ))
        .load::<(i64, Option<i64>)>(conn)
        .await
        .map(|rows| rows.into_iter().collect())
        .map_err(map_diesel_error)
}

fn mismatch_row(
    candidate: RewardCandidate,
    payout_records: &HashMap<i64, bool>,
    credit_records: &HashMap<i64, Option<i64>>,
) -> Result<Option<RewardReconciliationMismatchRowOutput>, PlatformRewardDashboardError> {
    let status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| PlatformRewardDashboardError::Database(error.to_string()))?;
    let has_payout_record = payout_records.contains_key(&candidate.id);
    let credit_notification_id = credit_records.get(&candidate.id).copied();
    Ok(reward_reconciliation_mismatch_row(
        RewardReconciliationMismatchRowFact {
            reward_candidate_id: candidate.id,
            course_id: candidate.course_id,
            student_user_id: candidate.student_user_id,
            status,
            approved_amount: candidate.approved_amount,
            updated_at: candidate.updated_at,
            has_payout_record,
            has_wallet_credit_record: credit_notification_id.is_some(),
            has_notification_record: credit_notification_id.flatten().is_some(),
        },
    ))
}
