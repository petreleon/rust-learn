use futures::future::BoxFuture;

use crate::application::rewards::list_course_candidates::{
    CourseRewardCandidate, CourseRewardCandidatesError, CourseRewardCandidatesQuery,
};

pub trait CourseRewardCandidatesUseCase: Send + Sync {
    fn list_course_reward_candidates(
        &self,
        actor_user_id: i32,
        course_id: i32,
        query: CourseRewardCandidatesQuery,
    ) -> BoxFuture<'_, Result<Vec<CourseRewardCandidate>, CourseRewardCandidatesError>>;
}
