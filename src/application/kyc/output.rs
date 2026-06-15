use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::domain::kyc::audit::KycAuditEventType;

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

pub(crate) struct KycSubmissionFact {
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

pub(crate) fn kyc_submission_output(fact: KycSubmissionFact) -> KycSubmissionOutput {
    KycSubmissionOutput {
        id: fact.id,
        user_id: fact.user_id,
        status: fact.status,
        legal_name: fact.legal_name,
        country_code: fact.country_code,
        document_type: fact.document_type,
        document_last4: fact.document_last4,
        evidence_reference: fact.evidence_reference,
        provider_reference: fact.provider_reference,
        reviewer_user_id: fact.reviewer_user_id,
        rejection_reason: fact.rejection_reason,
        submitted_at: fact.submitted_at,
        reviewed_at: fact.reviewed_at,
        created_at: fact.created_at,
        updated_at: fact.updated_at,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KycAuditEventOutput {
    pub id: i64,
    pub submission_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: KycAuditEventType,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

pub(crate) struct KycAuditEventFact {
    pub id: i64,
    pub submission_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: KycAuditEventType,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

pub(crate) fn kyc_audit_event_output(fact: KycAuditEventFact) -> KycAuditEventOutput {
    KycAuditEventOutput {
        id: fact.id,
        submission_id: fact.submission_id,
        actor_user_id: fact.actor_user_id,
        event_type: fact.event_type,
        from_status: fact.from_status,
        to_status: fact.to_status,
        reason: fact.reason,
        metadata: fact.metadata,
        created_at: fact.created_at,
    }
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
