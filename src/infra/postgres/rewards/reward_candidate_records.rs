use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_candidates;
use crate::models::reward_candidate::{NewRewardCandidate, RewardCandidate};

const DEFAULT_REWARD_CANDIDATE_LIMIT: i64 = 25;
const MAX_REWARD_CANDIDATE_LIMIT: i64 = 100;

#[derive(Debug, Clone, Default)]
pub(super) struct RewardCandidateFilter {
    pub course_id: Option<i32>,
    pub student_user_id: Option<i32>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl RewardCandidateFilter {
    fn limit(&self) -> i64 {
        self.limit
            .unwrap_or(DEFAULT_REWARD_CANDIDATE_LIMIT)
            .clamp(1, MAX_REWARD_CANDIDATE_LIMIT)
    }

    fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

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

pub(super) async fn list_candidates(
    conn: &mut AsyncPgConnection,
    filter: RewardCandidateFilter,
) -> QueryResult<Vec<RewardCandidate>> {
    let mut query = reward_candidates::table.into_boxed();
    let limit = filter.limit();
    let offset = filter.offset();

    if let Some(course_id) = filter.course_id {
        query = query.filter(reward_candidates::course_id.eq(course_id));
    }

    if let Some(student_user_id) = filter.student_user_id {
        query = query.filter(reward_candidates::student_user_id.eq(student_user_id));
    }

    if let Some(status) = filter.status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    query
        .order(reward_candidates::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<RewardCandidate>(conn)
        .await
}

pub(super) async fn count_candidates(
    conn: &mut AsyncPgConnection,
    filter: RewardCandidateFilter,
) -> QueryResult<i64> {
    let mut query = reward_candidates::table.into_boxed();

    if let Some(course_id) = filter.course_id {
        query = query.filter(reward_candidates::course_id.eq(course_id));
    }

    if let Some(student_user_id) = filter.student_user_id {
        query = query.filter(reward_candidates::student_user_id.eq(student_user_id));
    }

    if let Some(status) = filter.status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    query.count().get_result(conn).await
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
