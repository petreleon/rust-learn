#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCreationCommand {
    pub actor_user_id: i32,
    pub title: String,
    pub organization_ids: Vec<i32>,
}
