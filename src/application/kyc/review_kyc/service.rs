use futures::future::BoxFuture;

use crate::application::kyc::{
    KycDecisionCommand, KycError, KycReviewQueueOutput, KycSubmissionOutput,
};

pub trait KycReviewUseCase: Send + Sync {
    fn list_review_queue(
        &self,
        reviewer_user_id: i32,
    ) -> BoxFuture<'_, Result<KycReviewQueueOutput, KycError>>;

    fn decide_submission(
        &self,
        command: KycDecisionCommand,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>>;
}
