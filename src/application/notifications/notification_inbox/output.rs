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

pub(crate) struct NotificationFact {
    pub id: i64,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub read: bool,
}

pub(crate) fn notification_output(fact: NotificationFact) -> NotificationOutput {
    NotificationOutput {
        id: fact.id,
        user_id: fact.user_id,
        title: fact.title,
        body: fact.body,
        created_at: fact.created_at,
        read: fact.read,
    }
}
