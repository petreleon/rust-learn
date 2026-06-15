use crate::infra::postgres::schema::course_progress;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Insertable, Serialize)]
#[diesel(table_name = course_progress)]
pub struct CourseProgress {
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
    pub content_id: i32,
    pub viewed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = course_progress)]
pub struct NewCourseProgress {
    pub user_id: i32,
    pub course_id: i32,
    pub content_id: i32,
}
