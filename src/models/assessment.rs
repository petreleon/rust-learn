use crate::db::schema::{assessment_attempts, assessment_questions, assessments};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[diesel(table_name = assessments)]
pub struct Assessment {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub passing_score: i32,
    pub max_attempts: i32,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, Associations, Debug, Clone, Serialize)]
#[diesel(belongs_to(Assessment))]
#[diesel(table_name = assessment_questions)]
pub struct AssessmentQuestion {
    pub id: i32,
    pub assessment_id: i32,
    pub text: String,
    pub question_type: String,
    pub options: Option<Value>,
    pub correct_answer: Option<String>,
    pub points: i32,
    pub order: i32,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize)]
#[diesel(table_name = assessment_attempts)]
pub struct AssessmentAttempt {
    pub id: i32,
    pub assessment_id: i32,
    pub user_id: i32,
    pub score: Option<i32>,
    pub passed: Option<bool>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
