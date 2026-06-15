use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::kyc::{KycAuditEventOutput, KycError, KycSubmissionOutput};
use crate::domain::kyc::submission::{NormalizedKycDecision, NormalizedKycSubmission};

pub trait KycStore: AccessDecisionStore<Error = KycError> {
    fn get_user_kyc_verified(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, KycError>>;

    fn latest_submission_for_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<KycSubmissionOutput>, KycError>>;

    fn create_submission(
        &mut self,
        submission: NormalizedKycSubmission,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>>;

    fn list_review_queue(&mut self) -> BoxFuture<'_, Result<Vec<KycSubmissionOutput>, KycError>>;

    fn find_submission(
        &mut self,
        submission_id: i64,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>>;

    fn decide_submission(
        &mut self,
        submission_id: i64,
        reviewer_user_id: i32,
        from_status: String,
        decision: NormalizedKycDecision,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>>;

    fn list_submission_audit(
        &mut self,
        submission_id: i64,
    ) -> BoxFuture<'_, Result<Vec<KycAuditEventOutput>, KycError>>;
}
