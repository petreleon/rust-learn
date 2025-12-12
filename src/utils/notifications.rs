use crate::db::DbPool;
use diesel::prelude::*;

use crate::models::notification::{Notification, NewNotification};
use chrono::Utc;
use anyhow::Result;

#[derive(Clone)]
pub struct NotificationsState {
    pool: DbPool,
}

impl NotificationsState {
    /// Create a new NotificationsState from an existing DB pool.
    pub fn new(pool: DbPool) -> Self {
        NotificationsState { pool }
    }

    /// Send (add) a notification for a user.
    pub async fn send_notification(&self, user_id: i32, title: impl AsRef<str>, body: impl AsRef<str>) -> Result<i64> {
        use diesel_async::RunQueryDsl;
        let mut conn = self.pool.get().await.map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        let new = NewNotification { user_id: Some(user_id), title: title.as_ref(), body: body.as_ref() };
        let inserted_id = Notification::create(new, &mut conn).await?;
        Ok(inserted_id)
    }

    /// Get notifications for a user ordered by created_at desc.
    pub async fn get_notifications(&self, user_id: i32) -> Result<Vec<Notification>> {
        use diesel_async::RunQueryDsl;
        let mut conn = self.pool.get().await.map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        let rows = Notification::find_by_user_id(user_id, &mut conn).await?;
        Ok(rows)
    }

    /// Mark a notification read by its id.
    pub async fn mark_read(&self, user_id: i32, notification_id: i64) -> Result<()> {
        use diesel_async::RunQueryDsl;
        let mut conn = self.pool.get().await.map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        Notification::mark_as_read(user_id, notification_id, &mut conn).await?;
        Ok(())
    }

    /// Clear notifications for a user (delete).
    pub async fn clear(&self, user_id: i32) -> Result<()> {
        use diesel_async::RunQueryDsl;
        let mut conn = self.pool.get().await.map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        Notification::delete_by_user_id(user_id, &mut conn).await?;
        Ok(())
    }
}

impl From<DbPool> for NotificationsState {
    fn from(pool: DbPool) -> Self { NotificationsState::new(pool) }
}
