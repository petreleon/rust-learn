use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationOutput {
    pub id: i64,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub read: bool,
}
