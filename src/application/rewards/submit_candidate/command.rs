use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::shared::json::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct SubmitRewardCandidateCommand {
    pub student_user_id: i32,
    pub event_type: RewardEventType,
    pub idempotency_key: Option<String>,
    pub evidence: Option<JsonValue>,
}
