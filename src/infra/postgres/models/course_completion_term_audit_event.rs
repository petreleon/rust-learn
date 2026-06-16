use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::course_completion_term_audit_events;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone)]
#[diesel(table_name = course_completion_term_audit_events)]
pub struct CourseCompletionTermAuditEvent {
    pub id: i64,
    pub terms_id: i64,
    pub course_id: i32,
    pub actor_user_id: i32,
    pub event_type: String,
    pub previous_status: Option<String>,
    pub new_status: String,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = course_completion_term_audit_events)]
pub struct NewCourseCompletionTermAuditEvent {
    pub terms_id: i64,
    pub course_id: i32,
    pub actor_user_id: i32,
    pub event_type: String,
    pub previous_status: Option<String>,
    pub new_status: String,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub note: Option<String>,
}
