use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::notifications::preferences::NotificationPreferenceOutput;
use crate::application::notifications::save_preferences::SaveNotificationPreferencesCommand;

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationPreferencesRequest {
    pub email_enabled: bool,
    pub push_enabled: bool,
}

impl NotificationPreferencesRequest {
    pub fn into_command(self, user_id: i32) -> SaveNotificationPreferencesCommand {
        SaveNotificationPreferencesCommand {
            user_id,
            email_enabled: self.email_enabled,
            push_enabled: self.push_enabled,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationPreferencesResponse {
    pub user_id: i32,
    pub email_enabled: bool,
    pub push_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<NotificationPreferenceOutput> for NotificationPreferencesResponse {
    fn from(preferences: NotificationPreferenceOutput) -> Self {
        Self {
            user_id: preferences.user_id,
            email_enabled: preferences.email_enabled,
            push_enabled: preferences.push_enabled,
            updated_at: preferences.updated_at,
        }
    }
}
