use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::decide_teacher_candidate::{
    self, TeacherRewardCandidateDecisionCommand, TeacherRewardCandidateDecisionError,
    TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::teacher_reward_candidate_decision_store::PostgresTeacherRewardCandidateDecisionStore;

#[derive(Clone)]
pub struct PostgresTeacherRewardCandidateDecisionUseCase {
    pool: DbPool,
}

impl PostgresTeacherRewardCandidateDecisionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherRewardCandidateDecisionUseCase for PostgresTeacherRewardCandidateDecisionUseCase {
    fn decide_teacher_reward_candidate(
        &self,
        actor_user_id: i32,
        course_id: i32,
        candidate_id: i64,
        command: TeacherRewardCandidateDecisionCommand,
    ) -> BoxFuture<
        '_,
        Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherRewardCandidateDecisionStore::new(&mut conn);
            decide_teacher_candidate::decide_teacher_reward_candidate(
                &mut store,
                actor_user_id,
                course_id,
                candidate_id,
                command,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresTeacherRewardCandidateDecisionUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        TeacherRewardCandidateDecisionError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherRewardCandidateDecisionError::Connection(error.to_string()))
    }
}
