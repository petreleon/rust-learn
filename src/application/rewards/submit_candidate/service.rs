use futures::future::BoxFuture;

use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmissionError, RewardCandidateSubmissionOutput, SubmitRewardCandidateCommand,
};

pub trait RewardCandidateSubmissionUseCase: Send + Sync {
    fn submit_course_reward_candidate(
        &self,
        actor_user_id: i32,
        course_id: i32,
        command: SubmitRewardCandidateCommand,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>;

    fn submit_organization_reward_candidate(
        &self,
        actor_user_id: i32,
        organization_id: i32,
        course_id: i32,
        command: SubmitRewardCandidateCommand,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>;
}
