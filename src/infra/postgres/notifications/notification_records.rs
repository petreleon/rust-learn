use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::notifications;
use crate::models::notification::{NewNotification, Notification};

pub async fn insert_notification(
    conn: &mut AsyncPgConnection,
    new_notification: NewNotification<'_>,
) -> QueryResult<i64> {
    diesel::insert_into(notifications::table)
        .values(&new_notification)
        .returning(notifications::id)
        .get_result(conn)
        .await
}

pub async fn insert_notifications(
    conn: &mut AsyncPgConnection,
    new_notifications: &[NewNotification<'_>],
) -> QueryResult<usize> {
    if new_notifications.is_empty() {
        return Ok(0);
    }

    diesel::insert_into(notifications::table)
        .values(new_notifications)
        .execute(conn)
        .await
}

pub async fn list_user_notifications(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    limit: i64,
) -> QueryResult<Vec<Notification>> {
    notifications::table
        .filter(notifications::user_id.eq(user_id))
        .order(notifications::created_at.desc())
        .then_order_by(notifications::id.desc())
        .limit(limit)
        .load::<Notification>(conn)
        .await
}

pub async fn mark_user_notification_read(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    notification_id: i64,
) -> QueryResult<usize> {
    diesel::update(
        notifications::table.filter(
            notifications::id
                .eq(notification_id)
                .and(notifications::user_id.eq(user_id)),
        ),
    )
    .set(notifications::read.eq(true))
    .execute(conn)
    .await
}

pub async fn delete_user_notifications(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<usize> {
    diesel::delete(notifications::table.filter(notifications::user_id.eq(user_id)))
        .execute(conn)
        .await
}
