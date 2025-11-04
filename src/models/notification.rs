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
}
