use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmission, RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
    RewardCandidateSubmissionStore,
};
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_candidate_submission_mappers::map_reward_candidate_submission_error;
use crate::infra::postgres::rewards::reward_candidate_submission_mutation::submit_reward_candidate;
use crate::infra::postgres::schema::{courses, courses_organizations};

pub struct PostgresRewardCandidateSubmissionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardCandidateSubmissionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardCandidateSubmissionStore for PostgresRewardCandidateSubmissionStore<'_> {
    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<(), RewardCandidateSubmissionError>> {
        async move {
            courses::table
                .find(course_id)
                .select(courses::id)
                .first::<i32>(self.conn)
                .await
                .map(|_| ())
                .map_err(map_reward_candidate_submission_error)
        }
        .boxed()
    }

    fn course_attached_to_organization(
        &mut self,
        course_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>> {
        async move {
            diesel::select(exists(
                courses_organizations::table
                    .filter(courses_organizations::course_id.eq(course_id))
                    .filter(courses_organizations::organization_id.eq(organization_id)),
            ))
            .get_result(self.conn)
            .await
            .map_err(map_reward_candidate_submission_error)
        }
        .boxed()
    }

    fn can_submit_course_reward_event(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>> {
        async move {
            reward_authorization_access::can_submit_course_reward_event(
                self.conn,
                actor_user_id,
                course_id,
            )
            .await
            .map_err(|error| RewardCandidateSubmissionError::Database(error.to_string()))
        }
        .boxed()
    }

    fn can_submit_organization_course_reward_event(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>> {
        async move {
            reward_authorization_access::can_submit_organization_course_reward_event(
                self.conn,
                actor_user_id,
                organization_id,
            )
            .await
            .map_err(|error| RewardCandidateSubmissionError::Database(error.to_string()))
        }
        .boxed()
    }

    fn submit_reward_candidate(
        &mut self,
        submission: RewardCandidateSubmission,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>
    {
        async move { submit_reward_candidate(self.conn, submission).await }.boxed()
    }
}
