use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::kyc::{KycAuditEventOutput, KycStatusOutput, KycSubmissionOutput};

#[derive(Debug, Deserialize)]
pub(super) struct SubmitKycRequest {
    pub(super) legal_name: String,
    pub(super) country_code: String,
    pub(super) document_type: String,
    pub(super) document_last4: Option<String>,
    pub(super) evidence_reference: Option<String>,
    pub(super) provider_reference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct KycDecisionRequest {
    pub(super) status: String,
    pub(super) rejection_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct KycStatusResponse {
    pub(super) submission: Option<KycSubmissionResponse>,
    pub(super) user_kyc_verified: bool,
    pub(super) next_action: String,
}

#[derive(Debug, Serialize)]
pub(super) struct KycReviewQueueResponse {
    pub(super) submissions: Vec<KycSubmissionResponse>,
}

#[derive(Debug, Serialize)]
pub(super) struct KycSubmissionResponse {
    pub(super) id: i64,
    pub(super) user_id: i32,
    pub(super) status: String,
    pub(super) legal_name: String,
    pub(super) country_code: String,
    pub(super) document_type: String,
    pub(super) document_last4: Option<String>,
    pub(super) evidence_reference: Option<String>,
    pub(super) provider_reference: Option<String>,
    pub(super) reviewer_user_id: Option<i32>,
    pub(super) rejection_reason: Option<String>,
    pub(super) submitted_at: DateTime<Utc>,
    pub(super) reviewed_at: Option<DateTime<Utc>>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub(super) struct KycAuditEventResponse {
    pub(super) id: i64,
    pub(super) submission_id: i64,
    pub(super) actor_user_id: Option<i32>,
    pub(super) event_type: String,
    pub(super) from_status: Option<String>,
    pub(super) to_status: String,
    pub(super) reason: Option<String>,
    pub(super) metadata: Value,
    pub(super) created_at: DateTime<Utc>,
}

impl From<KycStatusOutput> for KycStatusResponse {
    fn from(output: KycStatusOutput) -> Self {
        Self {
            next_action: output.next_action,
            submission: output.submission.map(Into::into),
            user_kyc_verified: output.user_kyc_verified,
        }
    }
}

impl From<KycSubmissionOutput> for KycSubmissionResponse {
    fn from(submission: KycSubmissionOutput) -> Self {
        Self {
            country_code: submission.country_code,
            created_at: submission.created_at,
            document_last4: submission.document_last4,
            document_type: submission.document_type,
            evidence_reference: submission.evidence_reference,
            id: submission.id,
            legal_name: submission.legal_name,
            provider_reference: submission.provider_reference,
            rejection_reason: submission.rejection_reason,
            reviewed_at: submission.reviewed_at,
            reviewer_user_id: submission.reviewer_user_id,
            status: submission.status,
            submitted_at: submission.submitted_at,
            updated_at: submission.updated_at,
            user_id: submission.user_id,
        }
    }
}

impl From<KycAuditEventOutput> for KycAuditEventResponse {
    fn from(event: KycAuditEventOutput) -> Self {
        Self {
            actor_user_id: event.actor_user_id,
            created_at: event.created_at,
            event_type: event.event_type.as_str().to_string(),
            from_status: event.from_status,
            id: event.id,
            metadata: event.metadata,
            reason: event.reason,
            submission_id: event.submission_id,
            to_status: event.to_status,
        }
    }
}
