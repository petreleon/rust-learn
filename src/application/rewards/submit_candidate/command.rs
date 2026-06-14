use crate::shared::json::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct SubmitRewardCandidateCommand {
    pub student_user_id: i32,
    pub event_type: String,
    pub idempotency_key: Option<String>,
    pub evidence: Option<JsonValue>,
}
