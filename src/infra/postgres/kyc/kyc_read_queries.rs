use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::kyc::{KycAuditEventOutput, KycError, KycSubmissionOutput};
use crate::db::schema::{kyc_audit_events, kyc_submissions, users};
use crate::domain::kyc::submission::{KYC_STATUS_SUBMITTED, KYC_STATUS_UNDER_REVIEW};
use crate::infra::postgres::kyc::kyc_mappers::{
    kyc_audit_event_output_from_record, kyc_submission_output_from_record, map_error,
};
use crate::models::kyc_audit_event::KycAuditEvent;
use crate::models::kyc_submission::KycSubmission;

pub(super) async fn get_user_kyc_verified(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<bool, KycError> {
    users::table
        .find(user_id)
        .select(users::kyc_verified)
        .first::<bool>(conn)
        .await
        .map_err(map_error)
}

pub(super) async fn latest_submission_for_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<KycSubmissionOutput>, KycError> {
    kyc_submissions::table
        .filter(kyc_submissions::user_id.eq(user_id))
        .order(kyc_submissions::updated_at.desc())
        .then_order_by(kyc_submissions::id.desc())
        .first::<KycSubmission>(conn)
        .await
        .optional()
        .map(|submission| submission.map(kyc_submission_output_from_record))
        .map_err(map_error)
}

pub(super) async fn list_review_queue(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<KycSubmissionOutput>, KycError> {
    kyc_submissions::table
        .filter(kyc_submissions::status.eq_any([KYC_STATUS_SUBMITTED, KYC_STATUS_UNDER_REVIEW]))
        .order(kyc_submissions::submitted_at.asc())
        .then_order_by(kyc_submissions::id.asc())
        .limit(50)
        .load::<KycSubmission>(conn)
        .await
        .map(|items| {
            items
                .into_iter()
                .map(kyc_submission_output_from_record)
                .collect()
        })
        .map_err(map_error)
}

pub(super) async fn find_submission(
    conn: &mut AsyncPgConnection,
    submission_id: i64,
) -> Result<KycSubmissionOutput, KycError> {
    kyc_submissions::table
        .find(submission_id)
        .first::<KycSubmission>(conn)
        .await
        .map(kyc_submission_output_from_record)
        .map_err(map_error)
}

pub(super) async fn list_submission_audit(
    conn: &mut AsyncPgConnection,
    submission_id: i64,
) -> Result<Vec<KycAuditEventOutput>, KycError> {
    let events = kyc_audit_events::table
        .filter(kyc_audit_events::submission_id.eq(submission_id))
        .order(kyc_audit_events::created_at.asc())
        .then_order_by(kyc_audit_events::id.asc())
        .load::<KycAuditEvent>(conn)
        .await
        .map_err(map_error)?;
    events
        .into_iter()
        .map(kyc_audit_event_output_from_record)
        .collect()
}
