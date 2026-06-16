use chrono::{DateTime, Utc};

use crate::domain::learning::assessment::AssessmentQuestionOptions;
use crate::domain::rewards::candidate::event_type::REWARD_EVENT_ASSESSMENT_COMPLETION;

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
    pub questions: Vec<LearnerAssessmentQuestionOutput>,
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
pub struct LearnerAssessmentQuestionOutput {
    pub id: i32,
    pub assessment_id: i32,
    pub text: String,
    pub question_type: String,
    pub options: Option<AssessmentQuestionOptions>,
    pub points: i32,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentQuestionForScoring {
    pub id: i32,
    pub correct_answer: Option<String>,
    pub points: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentRewardHandoff {
    pub course_id: i32,
    pub assessment_id: i32,
    pub attempt_id: i32,
    pub user_id: i32,
    pub score: i32,
    pub total_points: i32,
    pub percentage: i32,
    pub passing_score: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentRewardHandoffOutput {
    pub status: AssessmentRewardHandoffStatus,
    pub event_type: String,
    pub candidate_id: Option<i64>,
    pub policy_id: Option<i64>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssessmentRewardHandoffStatus {
    NotEarned,
    Created,
    AlreadyExists,
    MissingPolicy,
    Failed,
}

impl AssessmentRewardHandoffOutput {
    pub fn not_earned() -> Self {
        Self::new(
            AssessmentRewardHandoffStatus::NotEarned,
            None,
            None,
            "Assessment was not passed; no reward candidate was created.",
        )
    }

    pub fn created(candidate_id: i64, policy_id: i64) -> Self {
        Self::new(
            AssessmentRewardHandoffStatus::Created,
            Some(candidate_id),
            Some(policy_id),
            "Reward review was queued for this assessment completion.",
        )
    }

    pub fn already_exists(candidate_id: Option<i64>, policy_id: Option<i64>) -> Self {
        Self::new(
            AssessmentRewardHandoffStatus::AlreadyExists,
            candidate_id,
            policy_id,
            "Reward review already exists for this assessment completion.",
        )
    }

    pub fn missing_policy() -> Self {
        Self::new(
            AssessmentRewardHandoffStatus::MissingPolicy,
            None,
            None,
            "No active assessment-completion reward policy covers this course.",
        )
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self::new(AssessmentRewardHandoffStatus::Failed, None, None, message)
    }

    fn new(
        status: AssessmentRewardHandoffStatus,
        candidate_id: Option<i64>,
        policy_id: Option<i64>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            status,
            event_type: REWARD_EVENT_ASSESSMENT_COMPLETION.to_string(),
            candidate_id,
            policy_id,
            message: message.into(),
        }
    }
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
