use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::application::kyc::{KycError, KycSubmissionOutput};
use crate::db::schema::{kyc_submissions, users};
use crate::domain::kyc::submission::{NormalizedKycDecision, NormalizedKycSubmission};
use crate::infra::postgres::kyc::kyc_audit::{record_decision_audit, record_submission_audit};
use crate::infra::postgres::kyc::kyc_mappers::{kyc_submission_output_from_record, map_error};
use crate::models::kyc_submission::{KycSubmission, NewKycSubmission};

pub(super) async fn create_submission(
    conn: &mut AsyncPgConnection,
    submission: NormalizedKycSubmission,
) -> Result<KycSubmissionOutput, KycError> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let submission = diesel::insert_into(kyc_submissions::table)
                .values(NewKycSubmission {
                    country_code: submission.country_code,
                    document_last4: submission.document_last4,
                    document_type: submission.document_type,
                    evidence_reference: submission.evidence_reference,
                    legal_name: submission.legal_name,
                    provider_reference: submission.provider_reference,
                    status: submission.status,
                    user_id: submission.user_id,
                })
                .get_result::<KycSubmission>(conn)
                .await?;
            record_submission_audit(conn, &submission).await?;
            Ok(submission)
        })
    })
    .await
    .map(kyc_submission_output_from_record)
    .map_err(map_error)
}

pub(super) async fn decide_submission(
    conn: &mut AsyncPgConnection,
    submission_id: i64,
    reviewer_user_id: i32,
    from_status: String,
    decision: NormalizedKycDecision,
) -> Result<KycSubmissionOutput, KycError> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let now = chrono::Utc::now();
            let updated = diesel::update(kyc_submissions::table.find(submission_id))
                .set((
                    kyc_submissions::status.eq(decision.status.as_str()),
                    kyc_submissions::reviewer_user_id.eq(Some(reviewer_user_id)),
                    kyc_submissions::rejection_reason.eq(decision.rejection_reason.clone()),
                    kyc_submissions::reviewed_at.eq(Some(now)),
                    kyc_submissions::updated_at.eq(now),
                ))
                .get_result::<KycSubmission>(conn)
                .await?;
            diesel::update(users::table.find(updated.user_id))
                .set(users::kyc_verified.eq(decision.verifies_user()))
                .execute(conn)
                .await?;
            record_decision_audit(conn, &updated, reviewer_user_id, from_status, decision).await?;
            Ok(updated)
        })
    })
    .await
    .map(kyc_submission_output_from_record)
    .map_err(map_error)
}
