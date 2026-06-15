use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::rewards::decide_amount::{
    RewardAmountDecisionCommand, RewardAmountDecisionOutput,
};
use crate::domain::rewards::candidate::evidence::RewardEvidence;

#[derive(Debug, Clone, Deserialize)]
pub struct RewardAmountDecisionRequest {
    pub status: String,
    pub approved_amount: Option<BigDecimal>,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardAmountDecisionResponse {
    pub id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: String,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub idempotency_key: String,
    pub evidence: RewardEvidence,
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

impl From<RewardAmountDecisionRequest> for RewardAmountDecisionCommand {
    fn from(request: RewardAmountDecisionRequest) -> Self {
        Self {
            status: request.status,
            approved_amount: request.approved_amount,
            decision_reason: request.decision_reason,
        }
    }
}

impl From<RewardAmountDecisionOutput> for RewardAmountDecisionResponse {
    fn from(candidate: RewardAmountDecisionOutput) -> Self {
        Self {
            id: candidate.id,
            course_id: candidate.course_id,
            student_user_id: candidate.student_user_id,
            submitter_user_id: candidate.submitter_user_id,
            source_scope: candidate.source_scope.as_str().to_string(),
            source_organization_id: candidate.source_organization_id,
            event_type: candidate.event_type.as_str().to_string(),
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
