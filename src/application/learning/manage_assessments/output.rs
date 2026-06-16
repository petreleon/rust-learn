use chrono::{DateTime, Utc};

use crate::domain::learning::assessment::AssessmentQuestionOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredAssessmentOutput {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub passing_score: i32,
    pub max_attempts: i32,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub questions: Vec<AuthoredAssessmentQuestionOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredAssessmentQuestionOutput {
    pub id: i32,
    pub assessment_id: i32,
    pub text: String,
    pub question_type: String,
    pub options: Option<AssessmentQuestionOptions>,
    pub correct_answer: Option<String>,
    pub points: i32,
    pub order: i32,
}
