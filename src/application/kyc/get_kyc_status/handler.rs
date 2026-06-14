use crate::application::kyc::{KycError, KycStatusOutput, KycStore};
use crate::domain::kyc::submission::next_action;

pub async fn get_my_status(
    store: &mut impl KycStore,
    user_id: i32,
) -> Result<KycStatusOutput, KycError> {
    let user_kyc_verified = store.get_user_kyc_verified(user_id).await?;
    let submission = store.latest_submission_for_user(user_id).await?;
    let latest_status = submission
        .as_ref()
        .map(|submission| submission.status.as_str());

    Ok(KycStatusOutput {
        next_action: next_action(user_kyc_verified, latest_status),
        submission,
        user_kyc_verified,
    })
}
