#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseRoleAssignmentCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub target_user_id: i32,
    pub role_name: String,
}
