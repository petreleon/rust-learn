use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::learning::assessment::{AssessmentAttemptOutput, AssessmentOutput};

#[derive(Debug, Clone, Serialize)]
pub struct AssessmentResponse {
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

impl From<AssessmentOutput> for AssessmentResponse {
    fn from(assessment: AssessmentOutput) -> Self {
        Self {
            id: assessment.id,
            course_id: assessment.course_id,
            title: assessment.title,
            description: assessment.description,
            passing_score: assessment.passing_score,
            max_attempts: assessment.max_attempts,
            published: assessment.published,
            created_at: assessment.created_at,
            updated_at: assessment.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AssessmentAttemptResponse {
    pub id: i32,
    pub assessment_id: i32,
    pub user_id: i32,
    pub score: Option<i32>,
    pub passed: Option<bool>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<AssessmentAttemptOutput> for AssessmentAttemptResponse {
    fn from(attempt: AssessmentAttemptOutput) -> Self {
        Self {
            id: attempt.id,
            assessment_id: attempt.assessment_id,
            user_id: attempt.user_id,
            score: attempt.score,
            passed: attempt.passed,
            started_at: attempt.started_at,
            completed_at: attempt.completed_at,
        }
    }
}
