use futures::future::BoxFuture;

use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidatesError, PlatformRewardCandidatesQuery, PlatformRewardCandidatesResponse,
};

pub trait PlatformRewardCandidatesUseCase: Send + Sync {
    fn list_platform_reward_candidates(
        &self,
        actor_user_id: i32,
        query: PlatformRewardCandidatesQuery,
    ) -> BoxFuture<'_, Result<PlatformRewardCandidatesResponse, PlatformRewardCandidatesError>>;
}
