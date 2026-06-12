use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::application::learning::assessment::{AssessmentAttemptOutput, AssessmentOutput};
use crate::application::learning::submit_assessment_attempt::{
    SubmitAssessmentAttemptCommand, SubmitAssessmentAttemptOutput,
};

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

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitAssessmentAttemptRequest {
    pub answers: HashMap<i32, String>,
}

impl SubmitAssessmentAttemptRequest {
    pub fn into_command(
        self,
        course_id: i32,
        assessment_id: i32,
        user_id: i32,
    ) -> SubmitAssessmentAttemptCommand {
        SubmitAssessmentAttemptCommand {
            course_id,
            assessment_id,
            user_id,
            answers: self.answers,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitAssessmentAttemptResponse {
    pub attempt: AssessmentAttemptResponse,
    pub score: i32,
    pub total_points: i32,
    pub percentage: i32,
    pub passed: bool,
}

impl From<SubmitAssessmentAttemptOutput> for SubmitAssessmentAttemptResponse {
    fn from(output: SubmitAssessmentAttemptOutput) -> Self {
        Self {
            attempt: AssessmentAttemptResponse::from(output.attempt),
            score: output.score,
            total_points: output.total_points,
            percentage: output.percentage,
            passed: output.passed,
        }
    }
}
