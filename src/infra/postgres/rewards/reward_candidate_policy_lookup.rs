use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{courses_organizations, reward_policies};
use crate::domain::rewards::policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};

pub(super) async fn active_reward_policy_ids_for_course_event(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> QueryResult<Vec<i64>> {
    let mut policy_ids = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .select(reward_policies::id)
        .load::<i64>(conn)
        .await?;

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        policy_ids.extend(
            reward_policies::table
                .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
                .filter(reward_policies::organization_id.eq_any(organization_ids))
                .filter(reward_policies::course_id.is_null())
                .filter(reward_policies::event_type.eq(event_type))
                .filter(reward_policies::active.eq(true))
                .select(reward_policies::id)
                .load::<i64>(conn)
                .await?,
        );
    }

    policy_ids.extend(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
            .filter(reward_policies::organization_id.is_null())
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true))
            .select(reward_policies::id)
            .load::<i64>(conn)
            .await?,
    );

    Ok(policy_ids)
}
