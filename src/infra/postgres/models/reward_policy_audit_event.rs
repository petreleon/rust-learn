use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::reward_policy_audit_events;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = reward_policy_audit_events)]
pub struct RewardPolicyAuditEvent {
    pub id: i64,
    pub reward_policy_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub previous_active: Option<bool>,
    pub new_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = reward_policy_audit_events)]
pub struct NewRewardPolicyAuditEvent {
    pub reward_policy_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub previous_active: Option<bool>,
    pub new_active: bool,
}
