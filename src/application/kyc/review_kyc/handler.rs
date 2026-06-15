use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::kyc::{
    KycDecisionCommand, KycError, KycReviewQueueOutput, KycStore, KycSubmissionOutput,
};
use crate::domain::kyc::submission::{ensure_can_decide, normalize_decision, KycDecisionInput};

const REVIEW_KYC_SUBMISSIONS: &str = "REVIEW_KYC_SUBMISSIONS";

pub async fn list_review_queue(
    store: &mut impl KycStore,
    reviewer_user_id: i32,
) -> Result<KycReviewQueueOutput, KycError> {
    ensure_review_permission(store, reviewer_user_id).await?;
    Ok(KycReviewQueueOutput {
        submissions: store.list_review_queue().await?,
    })
}

pub async fn decide_submission(
    store: &mut impl KycStore,
    command: KycDecisionCommand,
) -> Result<KycSubmissionOutput, KycError> {
    ensure_review_permission(store, command.reviewer_user_id).await?;
    let existing = store.find_submission(command.submission_id).await?;
    ensure_can_decide(&existing.status)?;
    let decision = normalize_decision(KycDecisionInput {
        rejection_reason: command.rejection_reason,
        status: command.status,
    })?;

    store
        .decide_submission(
            command.submission_id,
            command.reviewer_user_id,
            existing.status,
            decision,
        )
        .await
}

async fn ensure_review_permission(
    store: &mut impl KycStore,
    reviewer_user_id: i32,
) -> Result<(), KycError> {
    if store
        .can(
            AccessActor::user(reviewer_user_id),
            AccessAction::permission(REVIEW_KYC_SUBMISSIONS),
            AccessScope::platform(),
        )
        .await?
    {
        Ok(())
    } else {
        Err(KycError::PermissionDenied(
            REVIEW_KYC_SUBMISSIONS.to_string(),
        ))
    }
}
