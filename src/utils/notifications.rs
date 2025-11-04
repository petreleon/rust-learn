use crate::db::DbPool;
use diesel::prelude::*;
use crate::db::schema::notifications::dsl as N;
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
    pub fn send_notification(&self, user_id: i32, title: impl AsRef<str>, body: impl AsRef<str>) -> Result<i64> {
        let mut conn = self.pool.get()?;
        let new = NewNotification { user_id: Some(user_id), title: title.as_ref(), body: body.as_ref() };
        let inserted_id: i64 = diesel::insert_into(N::notifications)
            .values(&new)
            .returning(N::id)
            .get_result(&mut *conn)?;
        Ok(inserted_id)
    }

    /// Get notifications for a user ordered by created_at desc.
    pub fn get_notifications(&self, user_id: i32) -> Result<Vec<Notification>> {
        let mut conn = self.pool.get()?;
        let rows = N::notifications.filter(N::user_id.eq(user_id)).order(N::created_at.desc()).load::<Notification>(&mut *conn)?;
        Ok(rows)
    }

    /// Mark a notification read by its id.
    pub fn mark_read(&self, user_id: i32, notification_id: i64) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::update(N::notifications.filter(N::id.eq(notification_id).and(N::user_id.eq(user_id))))
            .set(N::read.eq(true))
            .execute(&mut *conn)?;
        Ok(())
    }

    /// Clear notifications for a user (delete).
    pub fn clear(&self, user_id: i32) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::delete(N::notifications.filter(N::user_id.eq(user_id))).execute(&mut *conn)?;
        Ok(())
    }
}

impl From<DbPool> for NotificationsState {
    fn from(pool: DbPool) -> Self { NotificationsState::new(pool) }
}
