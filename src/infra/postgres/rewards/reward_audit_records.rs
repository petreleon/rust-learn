use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_audit_events;
use crate::models::reward_audit_event::{NewRewardAuditEvent, RewardAuditEvent};

pub(super) async fn create_reward_audit_event(
    conn: &mut AsyncPgConnection,
    new_event: NewRewardAuditEvent,
) -> QueryResult<RewardAuditEvent> {
    diesel::insert_into(reward_audit_events::table)
        .values(&new_event)
        .get_result(conn)
        .await
}
