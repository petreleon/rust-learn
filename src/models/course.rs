use crate::db::schema::courses;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub const COURSE_STATUS_DRAFT: &str = "draft";
pub const COURSE_STATUS_SUBMITTED: &str = "submitted";
pub const COURSE_STATUS_NEEDS_CHANGES: &str = "needs_changes";
pub const COURSE_STATUS_APPROVED: &str = "approved";
pub const COURSE_STATUS_PUBLISHED: &str = "published";
pub const COURSE_STATUS_ARCHIVED: &str = "archived";
pub const COURSE_STATUS_SUSPENDED: &str = "suspended";

#[derive(Queryable, Identifiable, PartialEq, Debug, Selectable, Serialize)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub title: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = courses)]
pub struct UpdateCourse {
    pub title: Option<String>,
}
