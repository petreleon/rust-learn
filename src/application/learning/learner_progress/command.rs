#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveLearnerProgressCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub content_id: i32,
}
