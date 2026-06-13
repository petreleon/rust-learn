#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationAuditQuery {
    pub actor_user_id: i32,
    pub application_id: i64,
}
