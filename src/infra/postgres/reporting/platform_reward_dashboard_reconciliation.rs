use std::collections::HashMap;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_reward_dashboard::{
    classify_reward_reconciliation_mismatch, reconciliation_mismatch_candidate_statuses,
    PlatformRewardDashboardError, RewardReconciliationMismatchFacts,
    RewardReconciliationMismatchRowOutput,
};
use crate::db::schema::{reward_candidates, reward_payout_records, reward_wallet_credit_records};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::reporting::platform_reward_dashboard_summaries::map_diesel_error;
use crate::models::reward_candidate::RewardCandidate;

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
        .filter_map(|candidate| mismatch_row(candidate, &payout_records, &credit_records))
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
) -> Option<RewardReconciliationMismatchRowOutput> {
    let status = RewardCandidateStatus::parse(&candidate.status).ok()?;
    let has_payout_record = payout_records.contains_key(&candidate.id);
    let credit_notification_id = credit_records.get(&candidate.id).copied();
    let mismatch_type =
        classify_reward_reconciliation_mismatch(RewardReconciliationMismatchFacts {
            status,
            has_payout_record,
            has_wallet_credit_record: credit_notification_id.is_some(),
            has_notification_record: credit_notification_id.flatten().is_some(),
        })?;
    Some(RewardReconciliationMismatchRowOutput {
        reward_candidate_id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        status: candidate.status,
        mismatch_type: mismatch_type.as_str().to_string(),
        approved_amount: candidate.approved_amount.as_ref().map(ToString::to_string),
        updated_at: candidate.updated_at,
    })
}
