#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseLifecycleCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub status: String,
}
