use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::kyc_audit_events;
use crate::domain::kyc::audit::KycAuditEventType;
use crate::domain::kyc::submission::NormalizedKycDecision;
use crate::infra::postgres::models::kyc_audit_event::{KycAuditEvent, NewKycAuditEvent};
use crate::infra::postgres::models::kyc_submission::KycSubmission;

pub(super) async fn record_submission_audit(
    conn: &mut AsyncPgConnection,
    submission: &KycSubmission,
) -> diesel::QueryResult<KycAuditEvent> {
    diesel::insert_into(kyc_audit_events::table)
        .values(NewKycAuditEvent {
            actor_user_id: Some(submission.user_id),
            event_type: KycAuditEventType::Submitted.as_str().to_string(),
            from_status: None,
            metadata: audit_metadata(submission),
            reason: None,
            submission_id: submission.id,
            to_status: submission.status.clone(),
        })
        .get_result(conn)
        .await
}

pub(super) async fn record_decision_audit(
    conn: &mut AsyncPgConnection,
    submission: &KycSubmission,
    reviewer_user_id: i32,
    from_status: String,
    decision: NormalizedKycDecision,
) -> diesel::QueryResult<KycAuditEvent> {
    diesel::insert_into(kyc_audit_events::table)
        .values(NewKycAuditEvent {
            actor_user_id: Some(reviewer_user_id),
            event_type: KycAuditEventType::ReviewDecision.as_str().to_string(),
            from_status: Some(from_status),
            metadata: audit_metadata(submission),
            reason: decision.rejection_reason,
            submission_id: submission.id,
            to_status: submission.status.clone(),
        })
        .get_result(conn)
        .await
}

fn audit_metadata(submission: &KycSubmission) -> serde_json::Value {
    serde_json::json!({
        "country_code": submission.country_code,
        "document_type": submission.document_type,
        "has_document_last4": submission.document_last4.is_some(),
        "has_evidence_reference": submission.evidence_reference.is_some(),
        "has_provider_reference": submission.provider_reference.is_some()
    })
}
