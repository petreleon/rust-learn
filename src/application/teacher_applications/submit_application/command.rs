#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationSubmitCommand {
    pub actor_user_id: i32,
    pub requested_scope: String,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub organization_sponsor_id: Option<i32>,
    pub portfolio_links: Option<Vec<String>>,
    pub idempotency_key: Option<String>,
}
