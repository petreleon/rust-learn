use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::rewards::manage_reward_policy::RewardPolicyAuditEventOutput;

#[derive(Debug, Clone, Serialize)]
pub struct RewardPolicyAuditEventResponse {
    pub id: i64,
    pub reward_policy_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub previous_active: Option<bool>,
    pub new_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<RewardPolicyAuditEventOutput> for RewardPolicyAuditEventResponse {
    fn from(event: RewardPolicyAuditEventOutput) -> Self {
        Self {
            id: event.id,
            reward_policy_id: event.reward_policy_id,
            actor_user_id: event.actor_user_id,
            event_type: event.event_type.as_str().to_string(),
            previous_active: event.previous_active,
            new_active: event.new_active,
            created_at: event.created_at,
        }
    }
}
