use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::plan_payout::{RewardPayoutPlanError, RewardPayoutPolicy};
use crate::db::schema::{courses_organizations, reward_policies};
use crate::domain::rewards::policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::infra::postgres::rewards::reward_payout_plan_mappers::map_reward_payout_plan_error;
use crate::models::reward_policy::RewardPolicy;

pub(super) async fn active_reward_payout_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> Result<Option<RewardPayoutPolicy>, RewardPayoutPlanError> {
    if let Some(policy) = course_policy(conn, course_id, event_type).await? {
        return Ok(Some(policy));
    }

    if let Some(policy) = organization_policy(conn, course_id, event_type).await? {
        return Ok(Some(policy));
    }

    platform_policy(conn, event_type).await
}

async fn course_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> Result<Option<RewardPayoutPolicy>, RewardPayoutPlanError> {
    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()
        .map(|policy| policy.map(RewardPayoutPolicy::from))
        .map_err(map_reward_payout_plan_error)
}

async fn organization_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> Result<Option<RewardPayoutPolicy>, RewardPayoutPlanError> {
    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
        .map_err(map_reward_payout_plan_error)?;
    if organization_ids.is_empty() {
        return Ok(None);
    }

    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
        .filter(reward_policies::organization_id.eq_any(organization_ids))
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()
        .map(|policy| policy.map(RewardPayoutPolicy::from))
        .map_err(map_reward_payout_plan_error)
}

async fn platform_policy(
    conn: &mut AsyncPgConnection,
    event_type: &str,
) -> Result<Option<RewardPayoutPolicy>, RewardPayoutPlanError> {
    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
        .filter(reward_policies::organization_id.is_null())
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .order((
            reward_policies::version.desc(),
            reward_policies::created_at.desc(),
        ))
        .first::<RewardPolicy>(conn)
        .await
        .optional()
        .map(|policy| policy.map(RewardPayoutPolicy::from))
        .map_err(map_reward_payout_plan_error)
}
