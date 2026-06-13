use futures::future::BoxFuture;

use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmissionError, RewardCandidateSubmissionOutput, SubmitRewardCandidateCommand,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RewardCandidateSubmission {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub source_scope: String,
    pub source_organization_id: Option<i32>,
    pub command: SubmitRewardCandidateCommand,
}

pub trait RewardCandidateSubmissionStore {
    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<(), RewardCandidateSubmissionError>>;

    fn course_attached_to_organization(
        &mut self,
        course_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>>;

    fn can_submit_course_reward_event(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>>;

    fn can_submit_organization_course_reward_event(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>>;

    fn submit_reward_candidate(
        &mut self,
        submission: RewardCandidateSubmission,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>;
}
