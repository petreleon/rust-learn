use diesel::prelude::*;
use crate::db::schema::notifications;
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = notifications)]
pub struct Notification {
    pub id: i64,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub read: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = notifications)]
pub struct NewNotification<'a> {
    pub user_id: Option<i32>,
    pub title: &'a str,
    pub body: &'a str,
}

impl Notification {
    pub fn id(&self) -> i64 { self.id }

    pub fn create(new_notification: NewNotification, conn: &mut PgConnection) -> QueryResult<i64> {
        diesel::insert_into(notifications::table)
            .values(&new_notification)
            .returning(notifications::id)
            .get_result(conn)
    }

    pub fn find_by_user_id(user_id: i32, conn: &mut PgConnection) -> QueryResult<Vec<Notification>> {
        notifications::table
            .filter(notifications::user_id.eq(user_id))
            .order(notifications::created_at.desc())
            .load::<Notification>(conn)
    }

    pub fn mark_as_read(user_id: i32, notification_id: i64, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::update(notifications::table.filter(notifications::id.eq(notification_id).and(notifications::user_id.eq(user_id))))
            .set(notifications::read.eq(true))
            .execute(conn)
    }

    pub fn delete_by_user_id(user_id: i32, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(notifications::table.filter(notifications::user_id.eq(user_id)))
            .execute(conn)
    }
}
