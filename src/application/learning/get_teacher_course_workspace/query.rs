#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeacherCourseWorkspaceQuery {
    pub actor_user_id: i32,
    pub course_id: i32,
}

impl TeacherCourseWorkspaceQuery {
    pub fn new(actor_user_id: i32, course_id: i32) -> Self {
        Self {
            actor_user_id,
            course_id,
        }
    }
}
