use anyhow::Result;
use diesel_async::AsyncPgConnection;

use crate::infra::postgres::notifications::notification_records::{
    delete_user_notifications, insert_notification, insert_notifications,
    mark_user_notification_read,
};
use crate::models::notification::NewNotification;

use super::state::NotificationsState;

impl NotificationsState {
    /// Mark a notification read by its id.
    pub async fn mark_read(&self, user_id: i32, notification_id: i64) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        mark_user_notification_read(&mut conn, user_id, notification_id).await?;
        Ok(())
    }

    /// Clear notifications for a user (delete).
    pub async fn clear(&self, user_id: i32) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        delete_user_notifications(&mut conn, user_id).await?;
        Ok(())
    }
}

/// Create a single notification inside an existing transaction.
pub async fn create_notification(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    title: impl AsRef<str>,
    body: impl AsRef<str>,
) -> Result<i64> {
    let new = NewNotification {
        user_id: Some(user_id),
        title: title.as_ref(),
        body: body.as_ref(),
    };
    Ok(insert_notification(conn, new).await?)
}

/// Bulk-create notifications inside an existing transaction.
pub async fn create_notifications_bulk(
    conn: &mut AsyncPgConnection,
    notifications: &[NewNotification<'_>],
) -> Result<usize> {
    Ok(insert_notifications(conn, notifications).await?)
}
