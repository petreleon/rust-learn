use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::course_completion_terms;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone)]
#[diesel(table_name = course_completion_terms)]
pub struct CourseCompletionTerms {
    pub id: i64,
    pub course_id: i32,
    pub teacher_user_id: i32,
    pub organization_id: Option<i32>,
    pub version: i32,
    pub status: String,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub reward_policy_id: Option<i64>,
    pub proposed_by_user_id: i32,
    pub accepted_by_user_id: Option<i32>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub activated_at: Option<DateTime<Utc>>,
    pub superseded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = course_completion_terms)]
pub struct NewCourseCompletionTerms {
    pub course_id: i32,
    pub teacher_user_id: i32,
    pub organization_id: Option<i32>,
    pub version: i32,
    pub status: String,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub proposed_by_user_id: i32,
}
