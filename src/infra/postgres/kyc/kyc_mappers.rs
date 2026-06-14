use crate::application::kyc::{KycAuditEventOutput, KycError, KycSubmissionOutput};
use crate::models::kyc_audit_event::KycAuditEvent;
use crate::models::kyc_submission::KycSubmission;

impl From<KycSubmission> for KycSubmissionOutput {
    fn from(submission: KycSubmission) -> Self {
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

impl From<KycAuditEvent> for KycAuditEventOutput {
    fn from(event: KycAuditEvent) -> Self {
        Self {
            actor_user_id: event.actor_user_id,
            created_at: event.created_at,
            event_type: event.event_type,
            from_status: event.from_status,
            id: event.id,
            metadata: event.metadata,
            reason: event.reason,
            submission_id: event.submission_id,
            to_status: event.to_status,
        }
    }
}

pub(super) fn map_error(error: diesel::result::Error) -> KycError {
    match error {
        diesel::result::Error::NotFound => KycError::NotFound,
        other => KycError::Database(other.to_string()),
    }
}
