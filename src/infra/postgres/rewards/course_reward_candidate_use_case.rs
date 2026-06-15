use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_course_candidates::{
    self, CourseRewardCandidate, CourseRewardCandidatesError, CourseRewardCandidatesQuery,
    CourseRewardCandidatesUseCase,
};
use crate::infra::postgres::rewards::course_reward_candidate_store::PostgresCourseRewardCandidateStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseRewardCandidatesUseCase {
    pool: DbPool,
}

impl PostgresCourseRewardCandidatesUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseRewardCandidatesUseCase for PostgresCourseRewardCandidatesUseCase {
    fn list_course_reward_candidates(
        &self,
        actor_user_id: i32,
        course_id: i32,
        query: CourseRewardCandidatesQuery,
    ) -> BoxFuture<'_, Result<Vec<CourseRewardCandidate>, CourseRewardCandidatesError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseRewardCandidateStore::new(&mut conn);
            list_course_candidates::list_course_reward_candidates(
                &mut store,
                actor_user_id,
                course_id,
                query,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresCourseRewardCandidatesUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        CourseRewardCandidatesError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| CourseRewardCandidatesError::Connection(error.to_string()))
    }
}
