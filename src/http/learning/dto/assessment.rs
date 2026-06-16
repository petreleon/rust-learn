use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentRewardHandoffOutput,
    AssessmentRewardHandoffStatus, LearnerAssessmentQuestionOutput,
};
use crate::application::learning::submit_assessment_attempt::{
    SubmitAssessmentAttemptCommand, SubmitAssessmentAttemptOutput,
};
use crate::domain::learning::assessment::AssessmentQuestionOptions;

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
    pub questions: Vec<LearnerAssessmentQuestionResponse>,
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
            questions: assessment
                .questions
                .into_iter()
                .map(LearnerAssessmentQuestionResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LearnerAssessmentQuestionResponse {
    pub id: i32,
    pub assessment_id: i32,
    pub text: String,
    pub question_type: String,
    pub options: Option<AssessmentQuestionOptions>,
    pub points: i32,
    pub order: i32,
}

impl From<LearnerAssessmentQuestionOutput> for LearnerAssessmentQuestionResponse {
    fn from(question: LearnerAssessmentQuestionOutput) -> Self {
        Self {
            id: question.id,
            assessment_id: question.assessment_id,
            text: question.text,
            question_type: question.question_type,
            options: question.options,
            points: question.points,
            order: question.order,
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
    pub reward_handoff: AssessmentRewardHandoffResponse,
}

impl From<SubmitAssessmentAttemptOutput> for SubmitAssessmentAttemptResponse {
    fn from(output: SubmitAssessmentAttemptOutput) -> Self {
        Self {
            attempt: AssessmentAttemptResponse::from(output.attempt),
            score: output.score,
            total_points: output.total_points,
            percentage: output.percentage,
            passed: output.passed,
            reward_handoff: AssessmentRewardHandoffResponse::from(output.reward_handoff),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AssessmentRewardHandoffResponse {
    pub status: &'static str,
    pub event_type: String,
    pub candidate_id: Option<i64>,
    pub policy_id: Option<i64>,
    pub message: String,
}

impl From<AssessmentRewardHandoffOutput> for AssessmentRewardHandoffResponse {
    fn from(output: AssessmentRewardHandoffOutput) -> Self {
        Self {
            status: reward_handoff_status(output.status),
            event_type: output.event_type,
            candidate_id: output.candidate_id,
            policy_id: output.policy_id,
            message: output.message,
        }
    }
}

fn reward_handoff_status(status: AssessmentRewardHandoffStatus) -> &'static str {
    match status {
        AssessmentRewardHandoffStatus::NotEarned => "not_earned",
        AssessmentRewardHandoffStatus::Created => "created",
        AssessmentRewardHandoffStatus::AlreadyExists => "already_exists",
        AssessmentRewardHandoffStatus::MissingPolicy => "missing_policy",
        AssessmentRewardHandoffStatus::Failed => "failed",
    }
}
