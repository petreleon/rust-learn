use crate::db::schema::course_join_requests;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

pub const COURSE_JOIN_STATUS_PENDING: &str = "pending";
pub const COURSE_JOIN_STATUS_WAITLISTED: &str = "waitlisted";
pub const COURSE_JOIN_STATUS_APPROVED: &str = "approved";
pub const COURSE_JOIN_STATUS_REJECTED: &str = "rejected";

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = course_join_requests)]
pub struct CourseJoinRequest {
    pub id: i64,
    pub course_id: i32,
    pub requester_user_id: i32,
    pub status: String,
    pub reviewer_user_id: Option<i32>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = course_join_requests)]
pub struct NewCourseJoinRequest {
    pub course_id: i32,
    pub requester_user_id: i32,
    pub status: String,
}
