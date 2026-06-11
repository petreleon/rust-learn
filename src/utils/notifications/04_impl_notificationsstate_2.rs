impl NotificationsState {
    /// Mark a notification read by its id.
    pub async fn mark_read(&self, user_id: i32, notification_id: i64) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        Notification::mark_as_read(user_id, notification_id, &mut conn).await?;
        Ok(())
    }

    /// Clear notifications for a user (delete).
    pub async fn clear(&self, user_id: i32) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("DB Connection error: {}", e))?;
        Notification::delete_by_user_id(user_id, &mut conn).await?;
        Ok(())
    }
}
impl From<DbPool> for NotificationsState {
    fn from(pool: DbPool) -> Self {
        NotificationsState::new(pool)
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
    Ok(Notification::create(new, conn).await?)
}

/// Bulk-create notifications inside an existing transaction.
pub async fn create_notifications_bulk(
    conn: &mut AsyncPgConnection,
    notifications: &[NewNotification<'_>],
) -> Result<usize> {
    Ok(Notification::create_many(notifications, conn).await?)
}

#[cfg(test)]
mod tests;
