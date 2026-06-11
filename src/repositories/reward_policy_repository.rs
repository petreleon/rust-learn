use crate::db::schema::reward_policies;
use crate::models::reward_policy::{NewRewardPolicy, RewardPolicy};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

const DEFAULT_REWARD_POLICY_LIMIT: i64 = 25;
const MAX_REWARD_POLICY_LIMIT: i64 = 100;

mod activation;

pub use activation::deactivate_active_policies;

#[derive(Debug, Clone, Default)]
pub struct RewardPolicyFilter {
    pub scope_type: Option<String>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: Option<String>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl RewardPolicyFilter {
    fn limit(&self) -> i64 {
        self.limit
            .unwrap_or(DEFAULT_REWARD_POLICY_LIMIT)
            .clamp(1, MAX_REWARD_POLICY_LIMIT)
    }

    fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

pub async fn create_policy(
    conn: &mut AsyncPgConnection,
    new_policy: NewRewardPolicy,
) -> QueryResult<RewardPolicy> {
    diesel::insert_into(reward_policies::table)
        .values(&new_policy)
        .get_result(conn)
        .await
}

pub async fn list_policies(
    conn: &mut AsyncPgConnection,
    filter: RewardPolicyFilter,
) -> QueryResult<Vec<RewardPolicy>> {
    let mut query = reward_policies::table.into_boxed();
    let limit = filter.limit();
    let offset = filter.offset();

    if let Some(scope_type) = filter.scope_type {
        query = query.filter(reward_policies::scope_type.eq(scope_type));
    }

    if let Some(organization_id) = filter.organization_id {
        query = query.filter(reward_policies::organization_id.eq(Some(organization_id)));
    }

    if let Some(course_id) = filter.course_id {
        query = query.filter(reward_policies::course_id.eq(Some(course_id)));
    }

    if let Some(event_type) = filter.event_type {
        query = query.filter(reward_policies::event_type.eq(event_type));
    }

    if let Some(active) = filter.active {
        query = query.filter(reward_policies::active.eq(active));
    }

    query
        .order((
            reward_policies::created_at.desc(),
            reward_policies::version.desc(),
        ))
        .limit(limit)
        .offset(offset)
        .load::<RewardPolicy>(conn)
        .await
}

pub async fn next_policy_version(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    event_type: &str,
) -> QueryResult<i32> {
    let mut query = reward_policies::table
        .filter(reward_policies::scope_type.eq(scope_type))
        .filter(reward_policies::event_type.eq(event_type))
        .into_boxed();

    query = match organization_id {
        Some(organization_id) => {
            query.filter(reward_policies::organization_id.eq(Some(organization_id)))
        }
        None => query.filter(reward_policies::organization_id.is_null()),
    };
    query = match course_id {
        Some(course_id) => query.filter(reward_policies::course_id.eq(Some(course_id))),
        None => query.filter(reward_policies::course_id.is_null()),
    };

    let current = query
        .select(diesel::dsl::max(reward_policies::version))
        .first::<Option<i32>>(conn)
        .await?;

    Ok(current.unwrap_or(0) + 1)
}
