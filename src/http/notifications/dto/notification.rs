use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::notifications::notification_inbox::NotificationOutput;

#[derive(Debug, Clone, Serialize)]
pub struct NotificationResponse {
    pub id: i64,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub read: bool,
}

impl From<NotificationOutput> for NotificationResponse {
    fn from(notification: NotificationOutput) -> Self {
        Self {
            id: notification.id,
            user_id: notification.user_id,
            title: notification.title,
            body: notification.body,
            created_at: notification.created_at,
            read: notification.read,
        }
    }
}
