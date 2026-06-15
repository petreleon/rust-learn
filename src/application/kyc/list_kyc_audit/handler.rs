use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::kyc::{KycAuditEventOutput, KycAuditQuery, KycError, KycStore};

const REVIEW_KYC_SUBMISSIONS: &str = "REVIEW_KYC_SUBMISSIONS";

pub async fn list_submission_audit(
    store: &mut impl KycStore,
    query: KycAuditQuery,
) -> Result<Vec<KycAuditEventOutput>, KycError> {
    if !store
        .can(
            AccessActor::user(query.reviewer_user_id),
            AccessAction::permission(REVIEW_KYC_SUBMISSIONS),
            AccessScope::platform(),
        )
        .await?
    {
        return Err(KycError::PermissionDenied(
            REVIEW_KYC_SUBMISSIONS.to_string(),
        ));
    }
    store.find_submission(query.submission_id).await?;
    store.list_submission_audit(query.submission_id).await
}
