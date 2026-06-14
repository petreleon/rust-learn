#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationDecisionCommand {
    pub actor_user_id: i32,
    pub application_id: i64,
    pub status: String,
    pub decision_reason: Option<String>,
}
