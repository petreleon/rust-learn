#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LearnerCourseDetailQuery {
    pub actor_user_id: i32,
    pub course_id: i32,
}

impl LearnerCourseDetailQuery {
    pub fn new(actor_user_id: i32, course_id: i32) -> Self {
        Self {
            actor_user_id,
            course_id,
        }
    }
}
