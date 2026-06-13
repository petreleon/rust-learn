use futures::future::BoxFuture;

use crate::application::notifications::notification_inbox::{
    NotificationInboxError, NotificationOutput,
};

pub trait NotificationInboxUseCase: Send + Sync {
    fn list_notifications(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<NotificationOutput>, NotificationInboxError>>;

    fn mark_notification_read(
        &self,
        user_id: i32,
        notification_id: i64,
    ) -> BoxFuture<'_, Result<(), NotificationInboxError>>;

    fn clear_notifications(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<(), NotificationInboxError>>;
}
