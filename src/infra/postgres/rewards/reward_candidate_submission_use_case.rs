use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::submit_candidate::{
    self, RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
    RewardCandidateSubmissionUseCase, SubmitRewardCandidateCommand,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_candidate_submission_store::PostgresRewardCandidateSubmissionStore;

#[derive(Clone)]
pub struct PostgresRewardCandidateSubmissionUseCase {
    pool: DbPool,
}

impl PostgresRewardCandidateSubmissionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardCandidateSubmissionUseCase for PostgresRewardCandidateSubmissionUseCase {
    fn submit_course_reward_candidate(
        &self,
        actor_user_id: i32,
        course_id: i32,
        command: SubmitRewardCandidateCommand,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardCandidateSubmissionStore::new(&mut conn);
            submit_candidate::submit_course_reward_candidate(
                &mut store,
                actor_user_id,
                course_id,
                command,
            )
            .await
        }
        .boxed()
    }

    fn submit_organization_reward_candidate(
        &self,
        actor_user_id: i32,
        organization_id: i32,
        course_id: i32,
        command: SubmitRewardCandidateCommand,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardCandidateSubmissionStore::new(&mut conn);
            submit_candidate::submit_organization_reward_candidate(
                &mut store,
                actor_user_id,
                organization_id,
                course_id,
                command,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRewardCandidateSubmissionUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardCandidateSubmissionError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardCandidateSubmissionError::Connection(error.to_string()))
    }
}
