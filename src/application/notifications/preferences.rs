use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationPreferenceOutput {
    pub user_id: i32,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

impl NotificationPreferenceOutput {
    pub fn default_for_user(user_id: i32) -> Self {
        Self {
            user_id,
            email_enabled: true,
            push_enabled: false,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationPreferencesError {
    Database(String),
}
