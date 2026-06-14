use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct KycSubmissionOutput {
    pub id: i64,
    pub user_id: i32,
    pub status: String,
    pub legal_name: String,
    pub country_code: String,
    pub document_type: String,
    pub document_last4: Option<String>,
    pub evidence_reference: Option<String>,
    pub provider_reference: Option<String>,
    pub reviewer_user_id: Option<i32>,
    pub rejection_reason: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KycAuditEventOutput {
    pub id: i64,
    pub submission_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KycStatusOutput {
    pub submission: Option<KycSubmissionOutput>,
    pub user_kyc_verified: bool,
    pub next_action: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KycReviewQueueOutput {
    pub submissions: Vec<KycSubmissionOutput>,
}
