use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::models::reward_policy::RewardPolicy;
use crate::infra::postgres::schema::reward_policies;

pub async fn active_scope_policies(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    event_type: &str,
) -> QueryResult<Vec<RewardPolicy>> {
    scoped_active_query(scope_type, organization_id, course_id, event_type)
        .load::<RewardPolicy>(conn)
        .await
}

pub async fn set_policy_active(
    conn: &mut AsyncPgConnection,
    policy_id: i64,
    active: bool,
    updated_at: DateTime<Utc>,
) -> QueryResult<RewardPolicy> {
    diesel::update(reward_policies::table.find(policy_id))
        .set((
            reward_policies::active.eq(active),
            reward_policies::updated_at.eq(updated_at),
        ))
        .get_result(conn)
        .await
}

pub async fn deactivate_active_policies(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    event_type: &str,
    updated_at: DateTime<Utc>,
) -> QueryResult<usize> {
    match (organization_id, course_id) {
        (Some(organization_id), Some(course_id)) => {
            deactivate_for_course_org(
                conn,
                scope_type,
                organization_id,
                course_id,
                event_type,
                updated_at,
            )
            .await
        }
        (Some(organization_id), None) => {
            deactivate_for_org(conn, scope_type, organization_id, event_type, updated_at).await
        }
        (None, Some(course_id)) => {
            deactivate_for_course(conn, scope_type, course_id, event_type, updated_at).await
        }
        (None, None) => deactivate_global(conn, scope_type, event_type, updated_at).await,
    }
}

fn scoped_active_query<'a>(
    scope_type: &'a str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    event_type: &'a str,
) -> reward_policies::BoxedQuery<'a, diesel::pg::Pg> {
    let mut query = reward_policies::table
        .filter(reward_policies::scope_type.eq(scope_type))
        .filter(reward_policies::event_type.eq(event_type))
        .filter(reward_policies::active.eq(true))
        .into_boxed();

    query = match organization_id {
        Some(organization_id) => {
            query.filter(reward_policies::organization_id.eq(Some(organization_id)))
        }
        None => query.filter(reward_policies::organization_id.is_null()),
    };
    match course_id {
        Some(course_id) => query.filter(reward_policies::course_id.eq(Some(course_id))),
        None => query.filter(reward_policies::course_id.is_null()),
    }
}

async fn deactivate_for_course_org(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    organization_id: i32,
    course_id: i32,
    event_type: &str,
    updated_at: DateTime<Utc>,
) -> QueryResult<usize> {
    diesel::update(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(scope_type))
            .filter(reward_policies::organization_id.eq(Some(organization_id)))
            .filter(reward_policies::course_id.eq(Some(course_id)))
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true)),
    )
    .set((
        reward_policies::active.eq(false),
        reward_policies::updated_at.eq(updated_at),
    ))
    .execute(conn)
    .await
}

async fn deactivate_for_org(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    organization_id: i32,
    event_type: &str,
    updated_at: DateTime<Utc>,
) -> QueryResult<usize> {
    diesel::update(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(scope_type))
            .filter(reward_policies::organization_id.eq(Some(organization_id)))
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true)),
    )
    .set((
        reward_policies::active.eq(false),
        reward_policies::updated_at.eq(updated_at),
    ))
    .execute(conn)
    .await
}

async fn deactivate_for_course(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    course_id: i32,
    event_type: &str,
    updated_at: DateTime<Utc>,
) -> QueryResult<usize> {
    diesel::update(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(scope_type))
            .filter(reward_policies::organization_id.is_null())
            .filter(reward_policies::course_id.eq(Some(course_id)))
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true)),
    )
    .set((
        reward_policies::active.eq(false),
        reward_policies::updated_at.eq(updated_at),
    ))
    .execute(conn)
    .await
}

async fn deactivate_global(
    conn: &mut AsyncPgConnection,
    scope_type: &str,
    event_type: &str,
    updated_at: DateTime<Utc>,
) -> QueryResult<usize> {
    diesel::update(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(scope_type))
            .filter(reward_policies::organization_id.is_null())
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::event_type.eq(event_type))
            .filter(reward_policies::active.eq(true)),
    )
    .set((
        reward_policies::active.eq(false),
        reward_policies::updated_at.eq(updated_at),
    ))
    .execute(conn)
    .await
}
