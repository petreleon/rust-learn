use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmissionOutput, SubmitRewardCandidateCommand,
};
use crate::shared::json::JsonValue;

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitRewardCandidateRequest {
    pub student_user_id: i32,
    pub event_type: String,
    pub idempotency_key: Option<String>,
    pub evidence: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardCandidateSubmissionResponse {
    pub id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: String,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub idempotency_key: String,
    pub evidence: JsonValue,
    pub status: String,
    pub teacher_approver_user_id: Option<i32>,
    pub teacher_decision_reason: Option<String>,
    pub teacher_decided_at: Option<DateTime<Utc>>,
    pub amount_reviewer_user_id: Option<i32>,
    pub approved_amount: Option<BigDecimal>,
    pub amount_decision_reason: Option<String>,
    pub amount_decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SubmitRewardCandidateRequest> for SubmitRewardCandidateCommand {
    fn from(request: SubmitRewardCandidateRequest) -> Self {
        Self {
            student_user_id: request.student_user_id,
            event_type: request.event_type,
            idempotency_key: request.idempotency_key,
            evidence: request.evidence,
        }
    }
}

impl From<RewardCandidateSubmissionOutput> for RewardCandidateSubmissionResponse {
    fn from(candidate: RewardCandidateSubmissionOutput) -> Self {
        Self {
            id: candidate.id,
            course_id: candidate.course_id,
            student_user_id: candidate.student_user_id,
            submitter_user_id: candidate.submitter_user_id,
            source_scope: candidate.source_scope,
            source_organization_id: candidate.source_organization_id,
            event_type: candidate.event_type,
            idempotency_key: candidate.idempotency_key,
            evidence: candidate.evidence,
            status: candidate.status.as_str().to_string(),
            teacher_approver_user_id: candidate.teacher_approver_user_id,
            teacher_decision_reason: candidate.teacher_decision_reason,
            teacher_decided_at: candidate.teacher_decided_at,
            amount_reviewer_user_id: candidate.amount_reviewer_user_id,
            approved_amount: candidate.approved_amount,
            amount_decision_reason: candidate.amount_decision_reason,
            amount_decided_at: candidate.amount_decided_at,
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        }
    }
}
