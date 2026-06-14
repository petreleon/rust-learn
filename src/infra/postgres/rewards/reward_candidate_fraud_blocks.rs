use chrono::Utc;
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{courses_organizations, reward_fraud_blocks};
use crate::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::infra::postgres::rewards::reward_candidate_policy_lookup::active_reward_policy_ids_for_course_event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewardCandidateFraudBlockError {
    Blocked(String),
    NotFound,
    Database(String),
}

pub async fn ensure_no_active_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    teacher_user_ids: &[i32],
    course_id: i32,
    event_type: &str,
    source_organization_id: Option<i32>,
) -> Result<(), RewardCandidateFraudBlockError> {
    if !teacher_user_ids.is_empty()
        && has_active_teacher_reward_fraud_block(conn, teacher_user_ids).await?
    {
        return blocked("teacher reward activity is blocked by platform fraud controls");
    }

    if has_active_course_reward_fraud_block(conn, course_id).await? {
        return blocked("course reward activity is blocked by platform fraud controls");
    }

    let organization_ids =
        course_organization_ids_with_source(conn, course_id, source_organization_id).await?;
    if !organization_ids.is_empty()
        && has_active_organization_reward_fraud_block(conn, organization_ids.as_slice()).await?
    {
        return blocked("organization reward activity is blocked by platform fraud controls");
    }

    let active_policy_ids = active_reward_policy_ids_for_course_event(conn, course_id, event_type)
        .await
        .map_err(map_fraud_block_diesel_error)?;
    if !active_policy_ids.is_empty()
        && has_active_reward_policy_fraud_block(conn, active_policy_ids.as_slice()).await?
    {
        return blocked("reward policy activity is blocked by platform fraud controls");
    }

    Ok(())
}

async fn course_organization_ids_with_source(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    source_organization_id: Option<i32>,
) -> Result<Vec<i32>, RewardCandidateFraudBlockError> {
    let mut organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
        .map_err(map_fraud_block_diesel_error)?;

    if let Some(source_organization_id) = source_organization_id {
        if !organization_ids.contains(&source_organization_id) {
            organization_ids.push(source_organization_id);
        }
    }

    Ok(organization_ids)
}

async fn has_active_teacher_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    teacher_user_ids: &[i32],
) -> Result<bool, RewardCandidateFraudBlockError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_TEACHER))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::teacher_user_id.eq_any(teacher_user_ids)),
    ))
    .get_result(conn)
    .await
    .map_err(map_fraud_block_diesel_error)
}

async fn has_active_course_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<bool, RewardCandidateFraudBlockError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_COURSE))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::course_id.eq(course_id)),
    ))
    .get_result(conn)
    .await
    .map_err(map_fraud_block_diesel_error)
}

async fn has_active_organization_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    organization_ids: &[i32],
) -> Result<bool, RewardCandidateFraudBlockError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::organization_id.eq_any(organization_ids)),
    ))
    .get_result(conn)
    .await
    .map_err(map_fraud_block_diesel_error)
}

async fn has_active_reward_policy_fraud_block(
    conn: &mut AsyncPgConnection,
    reward_policy_ids: &[i64],
) -> Result<bool, RewardCandidateFraudBlockError> {
    diesel::select(exists(
        reward_fraud_blocks::table
            .filter(reward_fraud_blocks::scope_type.eq(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY))
            .filter(reward_fraud_blocks::revoked_at.is_null())
            .filter(
                reward_fraud_blocks::expires_at
                    .is_null()
                    .or(reward_fraud_blocks::expires_at.gt(Utc::now())),
            )
            .filter(reward_fraud_blocks::reward_policy_id.eq_any(reward_policy_ids)),
    ))
    .get_result(conn)
    .await
    .map_err(map_fraud_block_diesel_error)
}

fn blocked(message: &str) -> Result<(), RewardCandidateFraudBlockError> {
    Err(RewardCandidateFraudBlockError::Blocked(message.to_string()))
}

fn map_fraud_block_diesel_error(error: diesel::result::Error) -> RewardCandidateFraudBlockError {
    match error {
        diesel::result::Error::NotFound => RewardCandidateFraudBlockError::NotFound,
        other => RewardCandidateFraudBlockError::Database(other.to_string()),
    }
}
