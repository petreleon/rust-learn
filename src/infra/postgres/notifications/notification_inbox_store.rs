use futures::future::{BoxFuture, FutureExt};

use crate::application::notifications::notification_inbox::{
    notification_output, NotificationFact, NotificationInboxError, NotificationOutput,
    NOTIFICATION_LIST_LIMIT,
};
use crate::application::notifications::ports::NotificationInboxStore;
use crate::infra::postgres::models::notification::Notification;
use crate::infra::postgres::notifications::notification_records::{
    delete_user_notifications, list_user_notifications, mark_user_notification_read,
};

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
            list_user_notifications(self.conn, user_id, NOTIFICATION_LIST_LIMIT)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(notification_fact_from_record)
                        .map(notification_output)
                        .collect()
                })
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
            mark_user_notification_read(self.conn, user_id, notification_id)
                .await
                .map(|_| ())
                .map_err(map_notification_inbox_error)
        }
        .boxed()
    }

    fn clear(&mut self, user_id: i32) -> BoxFuture<'_, Result<(), NotificationInboxError>> {
        async move {
            delete_user_notifications(self.conn, user_id)
                .await
                .map(|_| ())
                .map_err(map_notification_inbox_error)
        }
        .boxed()
    }
}

fn notification_fact_from_record(notification: Notification) -> NotificationFact {
    NotificationFact {
        id: notification.id,
        user_id: notification.user_id,
        title: notification.title,
        body: notification.body,
        created_at: notification.created_at,
        read: notification.read,
    }
}

fn map_notification_inbox_error(error: diesel::result::Error) -> NotificationInboxError {
    NotificationInboxError::Database(error.to_string())
}
