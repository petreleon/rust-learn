use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_candidates;
use crate::models::reward_candidate::{NewRewardCandidate, RewardCandidate};

pub(super) async fn create_candidate(
    conn: &mut AsyncPgConnection,
    new_candidate: NewRewardCandidate,
) -> QueryResult<RewardCandidate> {
    diesel::insert_into(reward_candidates::table)
        .values(&new_candidate)
        .get_result(conn)
        .await
}

pub(super) async fn find_candidate(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    reward_candidates::table
        .find(candidate_id)
        .first::<RewardCandidate>(conn)
        .await
}

pub(super) async fn find_candidate_by_idempotency_key(
    conn: &mut AsyncPgConnection,
    idempotency_key: &str,
) -> QueryResult<Option<RewardCandidate>> {
    reward_candidates::table
        .filter(reward_candidates::idempotency_key.eq(idempotency_key))
        .first::<RewardCandidate>(conn)
        .await
        .optional()
}

pub(super) async fn update_teacher_decision(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    approver_user_id: i32,
    status: &str,
    decision_reason: Option<&str>,
    decided_at: DateTime<Utc>,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(status),
            reward_candidates::teacher_approver_user_id.eq(Some(approver_user_id)),
            reward_candidates::teacher_decision_reason.eq(decision_reason),
            reward_candidates::teacher_decided_at.eq(Some(decided_at)),
            reward_candidates::updated_at.eq(decided_at),
        ))
        .get_result(conn)
        .await
}

pub(super) async fn update_amount_decision(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    reviewer_user_id: i32,
    status: &str,
    approved_amount: Option<BigDecimal>,
    decision_reason: Option<&str>,
    decided_at: DateTime<Utc>,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(status),
            reward_candidates::amount_reviewer_user_id.eq(Some(reviewer_user_id)),
            reward_candidates::approved_amount.eq(approved_amount),
            reward_candidates::amount_decision_reason.eq(decision_reason),
            reward_candidates::amount_decided_at.eq(Some(decided_at)),
            reward_candidates::updated_at.eq(decided_at),
        ))
        .get_result(conn)
        .await
}
