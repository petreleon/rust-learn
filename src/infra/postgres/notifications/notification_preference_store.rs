use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::notifications::ports::NotificationPreferenceStore;
use crate::application::notifications::preferences::{
    notification_preference_output, NotificationPreferenceFact, NotificationPreferenceOutput,
    NotificationPreferencesError,
};
use crate::application::notifications::save_preferences::SaveNotificationPreferencesCommand;
use crate::infra::postgres::models::notification_preferences::{
    NotificationPreferences, UpsertNotificationPreferences,
};
use crate::infra::postgres::schema::user_notification_preferences;

pub struct PostgresNotificationPreferenceStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresNotificationPreferenceStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl NotificationPreferenceStore for PostgresNotificationPreferenceStore<'_> {
    fn find_by_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<NotificationPreferenceOutput>, NotificationPreferencesError>>
    {
        async move {
            user_notification_preferences::table
                .filter(user_notification_preferences::user_id.eq(user_id))
                .first::<NotificationPreferences>(self.conn)
                .await
                .optional()
                .map(|row| {
                    row.map(notification_preference_fact_from_record)
                        .map(notification_preference_output)
                })
                .map_err(map_notification_preferences_error)
        }
        .boxed()
    }

    fn save(
        &mut self,
        command: SaveNotificationPreferencesCommand,
    ) -> BoxFuture<'_, Result<NotificationPreferenceOutput, NotificationPreferencesError>> {
        async move {
            let _ = diesel::delete(
                user_notification_preferences::table
                    .filter(user_notification_preferences::user_id.eq(command.user_id)),
            )
            .execute(self.conn)
            .await;

            let payload = UpsertNotificationPreferences {
                user_id: command.user_id,
                email_enabled: command.email_enabled,
                push_enabled: command.push_enabled,
            };

            diesel::insert_into(user_notification_preferences::table)
                .values(&payload)
                .get_result::<NotificationPreferences>(self.conn)
                .await
                .map(notification_preference_fact_from_record)
                .map(notification_preference_output)
                .map_err(map_notification_preferences_error)
        }
        .boxed()
    }
}

fn notification_preference_fact_from_record(
    preferences: NotificationPreferences,
) -> NotificationPreferenceFact {
    NotificationPreferenceFact {
        user_id: preferences.user_id,
        email_enabled: preferences.email_enabled,
        push_enabled: preferences.push_enabled,
        updated_at: preferences.updated_at,
    }
}

fn map_notification_preferences_error(
    error: diesel::result::Error,
) -> NotificationPreferencesError {
    NotificationPreferencesError::Database(error.to_string())
}
