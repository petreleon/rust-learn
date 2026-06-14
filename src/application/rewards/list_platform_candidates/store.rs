use futures::future::BoxFuture;

use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidateCourseSummary, PlatformRewardCandidateRecord,
    PlatformRewardCandidateUserSummary, PlatformRewardCandidatesError,
};

pub trait PlatformRewardCandidateStore {
    fn can_view_reward_audit(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRewardCandidatesError>>;

    fn can_approve_reward_amount(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRewardCandidatesError>>;

    fn list_candidate_records(
        &mut self,
        status: Option<String>,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardCandidateRecord>, PlatformRewardCandidatesError>>;

    fn count_candidate_records(
        &mut self,
        status: Option<String>,
    ) -> BoxFuture<'_, Result<i64, PlatformRewardCandidatesError>>;

    fn load_user_summaries(
        &mut self,
        user_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardCandidateUserSummary>, PlatformRewardCandidatesError>>;

    fn load_course_summaries(
        &mut self,
        course_ids: Vec<i32>,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRewardCandidateCourseSummary>, PlatformRewardCandidatesError>,
    >;
}
