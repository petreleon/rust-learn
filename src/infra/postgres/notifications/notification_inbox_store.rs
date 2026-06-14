use futures::future::{BoxFuture, FutureExt};

use crate::application::notifications::notification_inbox::{
    NotificationInboxError, NotificationOutput, NOTIFICATION_LIST_LIMIT,
};
use crate::application::notifications::ports::NotificationInboxStore;
use crate::models::notification::Notification;

pub struct PostgresNotificationInboxStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresNotificationInboxStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl NotificationInboxStore for PostgresNotificationInboxStore<'_> {
    fn list(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<NotificationOutput>, NotificationInboxError>> {
        async move {
            Notification::find_by_user_id(user_id, NOTIFICATION_LIST_LIMIT, self.conn)
                .await
                .map(|rows| rows.into_iter().map(NotificationOutput::from).collect())
                .map_err(map_notification_inbox_error)
        }
        .boxed()
    }

    fn mark_read(
        &mut self,
        user_id: i32,
        notification_id: i64,
    ) -> BoxFuture<'_, Result<(), NotificationInboxError>> {
        async move {
            Notification::mark_as_read(user_id, notification_id, self.conn)
                .await
                .map(|_| ())
                .map_err(map_notification_inbox_error)
        }
        .boxed()
    }

    fn clear(&mut self, user_id: i32) -> BoxFuture<'_, Result<(), NotificationInboxError>> {
        async move {
            Notification::delete_by_user_id(user_id, self.conn)
                .await
                .map(|_| ())
                .map_err(map_notification_inbox_error)
        }
        .boxed()
    }
}

impl From<Notification> for NotificationOutput {
    fn from(notification: Notification) -> Self {
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

fn map_notification_inbox_error(error: diesel::result::Error) -> NotificationInboxError {
    NotificationInboxError::Database(error.to_string())
}
