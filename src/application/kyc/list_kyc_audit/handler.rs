use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::kyc::{KycAuditEventOutput, KycAuditQuery, KycError, KycStore};
use crate::domain::access_control::permissions::Permissions;

pub async fn list_submission_audit(
    store: &mut impl KycStore,
    query: KycAuditQuery,
) -> Result<Vec<KycAuditEventOutput>, KycError> {
    let permission = Permissions::REVIEW_KYC_SUBMISSIONS.to_string();
    if !store
        .can(
            AccessActor::user(query.reviewer_user_id),
            AccessAction::permission(permission.clone()),
            AccessScope::platform(),
        )
        .await?
    {
        return Err(KycError::PermissionDenied(permission));
    }
    store.find_submission(query.submission_id).await?;
    store.list_submission_audit(query.submission_id).await
}
