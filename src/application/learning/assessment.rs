use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentOutput {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentAttemptOutput {
    pub id: i32,
    pub assessment_id: i32,
    pub user_id: i32,
    pub score: Option<i32>,
    pub passed: Option<bool>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentQuestionForScoring {
    pub id: i32,
    pub correct_answer: Option<String>,
    pub points: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedAssessmentAttempt {
    pub assessment_id: i32,
    pub user_id: i32,
    pub score: i32,
    pub passed: bool,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssessmentReadError {
    Connection(String),
    Database(String),
}
