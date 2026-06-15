use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::models::reward_audit_event::{NewRewardAuditEvent, RewardAuditEvent};
use crate::infra::postgres::schema::reward_audit_events;

pub async fn create_reward_audit_event(
    conn: &mut AsyncPgConnection,
    new_event: NewRewardAuditEvent,
) -> QueryResult<RewardAuditEvent> {
    diesel::insert_into(reward_audit_events::table)
        .values(&new_event)
        .get_result(conn)
        .await
}

pub async fn list_reward_audit_events(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> QueryResult<Vec<RewardAuditEvent>> {
    reward_audit_events::table
        .filter(reward_audit_events::reward_candidate_id.eq(reward_candidate_id))
        .order((
            reward_audit_events::created_at.asc(),
            reward_audit_events::id.asc(),
        ))
        .load::<RewardAuditEvent>(conn)
        .await
}
