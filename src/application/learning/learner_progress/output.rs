use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerProgressOutput {
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
    pub content_id: i32,
    pub viewed_at: DateTime<Utc>,
}
