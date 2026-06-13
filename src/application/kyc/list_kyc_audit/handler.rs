use crate::application::kyc::{KycAuditEventOutput, KycAuditQuery, KycError, KycStore};

pub async fn list_submission_audit(
    store: &mut impl KycStore,
    query: KycAuditQuery,
) -> Result<Vec<KycAuditEventOutput>, KycError> {
    let permission = "REVIEW_KYC_SUBMISSIONS";
    if !store.can_review_kyc(query.reviewer_user_id).await? {
        return Err(KycError::PermissionDenied(permission.to_string()));
    }
    store.find_submission(query.submission_id).await?;
    store.list_submission_audit(query.submission_id).await
}
