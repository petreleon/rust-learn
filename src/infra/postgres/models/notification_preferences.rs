use crate::infra::postgres::schema::user_notification_preferences;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize)]
#[diesel(table_name = user_notification_preferences)]
pub struct NotificationPreferences {
    pub user_id: i32,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = user_notification_preferences)]
pub struct UpsertNotificationPreferences {
    pub user_id: i32,
    pub email_enabled: bool,
    pub push_enabled: bool,
}
