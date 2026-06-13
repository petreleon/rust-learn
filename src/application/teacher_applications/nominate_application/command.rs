#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationNominationCommand {
    pub actor_user_id: i32,
    pub organization_id: i32,
    pub applicant_user_id: i32,
    pub requested_scope: Option<String>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub portfolio_links: Option<Vec<String>>,
    pub idempotency_key: Option<String>,
}
