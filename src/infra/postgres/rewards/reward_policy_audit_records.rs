use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::models::reward_policy_audit_event::{
    NewRewardPolicyAuditEvent, RewardPolicyAuditEvent,
};
use crate::infra::postgres::schema::reward_policy_audit_events;

pub async fn insert_policy_audit_event(
    conn: &mut AsyncPgConnection,
    event: NewRewardPolicyAuditEvent,
) -> QueryResult<RewardPolicyAuditEvent> {
    diesel::insert_into(reward_policy_audit_events::table)
        .values(&event)
        .get_result(conn)
        .await
}

pub async fn list_policy_audit_events(
    conn: &mut AsyncPgConnection,
    policy_id: i64,
) -> QueryResult<Vec<RewardPolicyAuditEvent>> {
    reward_policy_audit_events::table
        .filter(reward_policy_audit_events::reward_policy_id.eq(policy_id))
        .order((
            reward_policy_audit_events::created_at.desc(),
            reward_policy_audit_events::id.desc(),
        ))
        .load::<RewardPolicyAuditEvent>(conn)
        .await
}
