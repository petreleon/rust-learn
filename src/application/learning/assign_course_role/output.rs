#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseRoleAssignmentOutput {
    pub course_id: i32,
    pub target_user_id: i32,
    pub role_name: String,
}
