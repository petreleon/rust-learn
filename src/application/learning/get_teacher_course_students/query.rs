#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseStudentsQuery {
    pub actor_user_id: i32,
    pub course_id: i32,
}

impl TeacherCourseStudentsQuery {
    pub fn new(actor_user_id: i32, course_id: i32) -> Self {
        Self {
            actor_user_id,
            course_id,
        }
    }
}
