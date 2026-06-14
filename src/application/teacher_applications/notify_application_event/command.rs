#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationNotificationCommand {
    pub application_id: i64,
    pub applicant_user_id: i32,
    pub requested_organization_id: Option<i32>,
    pub organization_sponsor_id: Option<i32>,
    pub status: String,
    pub requested_scope: String,
    pub event_type: String,
    pub reason: Option<String>,
}
