use crate::application::kyc::{KycError, KycStatusOutput, KycStore, SubmitKycCommand};
use crate::domain::kyc::submission::{
    build_submission, ensure_can_submit, next_action, KycSubmissionInput,
};

pub async fn submit_my_kyc(
    store: &mut impl KycStore,
    command: SubmitKycCommand,
) -> Result<KycStatusOutput, KycError> {
    let user_kyc_verified = store.get_user_kyc_verified(command.user_id).await?;
    let latest = store.latest_submission_for_user(command.user_id).await?;
    ensure_can_submit(
        user_kyc_verified,
        latest.as_ref().map(|submission| submission.status.as_str()),
    )?;

    let submission = build_submission(
        command.user_id,
        KycSubmissionInput {
            country_code: command.country_code,
            document_last4: command.document_last4,
            document_type: command.document_type,
            evidence_reference: command.evidence_reference,
            legal_name: command.legal_name,
            provider_reference: command.provider_reference,
        },
    )?;
    let submission = store.create_submission(submission).await?;

    Ok(KycStatusOutput {
        next_action: next_action(false, Some(submission.status.as_str())),
        submission: Some(submission),
        user_kyc_verified: false,
    })
}
