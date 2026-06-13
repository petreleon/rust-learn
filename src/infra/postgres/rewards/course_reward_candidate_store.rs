use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_course_candidates::{
    CourseRewardCandidate, CourseRewardCandidatesError, CourseRewardCandidatesFilter,
};
use crate::application::rewards::ports::CourseRewardCandidateStore;
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, reward_candidates};
use crate::infra::postgres::rewards::course_reward_candidate_mappers::map_course_reward_candidate_error;
use crate::models::reward_candidate::RewardCandidate;
use crate::repositories::course_repository::user_permission_course_request;

pub struct PostgresCourseRewardCandidateStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseRewardCandidateStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseRewardCandidateStore for PostgresCourseRewardCandidateStore<'_> {
    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseRewardCandidatesError>> {
        async move {
            courses::table
                .find(course_id)
                .select(courses::id)
                .first::<i32>(self.conn)
                .await
                .map(|_| ())
                .map_err(map_course_reward_candidate_error)
        }
        .boxed()
    }

    fn can_approve_student_reward_candidate(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>> {
        self.course_permission(
            actor_user_id,
            course_id,
            Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
        )
    }

    fn can_manage_course_reward_rules(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>> {
        self.course_permission(
            actor_user_id,
            course_id,
            Permissions::MANAGE_COURSE_REWARD_RULES,
        )
    }

    fn can_view_course_reward_status(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>> {
        self.course_permission(
            actor_user_id,
            course_id,
            Permissions::VIEW_COURSE_REWARD_STATUS,
        )
    }

    fn list_course_reward_candidates(
        &mut self,
        filter: CourseRewardCandidatesFilter,
    ) -> BoxFuture<'_, Result<Vec<CourseRewardCandidate>, CourseRewardCandidatesError>> {
        async move { list_candidates(self.conn, filter).await }.boxed()
    }
}

impl PostgresCourseRewardCandidateStore<'_> {
    fn course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: Permissions,
    ) -> BoxFuture<'_, Result<bool, CourseRewardCandidatesError>> {
        async move {
            user_permission_course_request(
                self.conn,
                actor_user_id,
                course_id,
                &permission.to_string(),
            )
            .await
            .map_err(map_course_reward_candidate_error)
        }
        .boxed()
    }
}

async fn list_candidates(
    conn: &mut AsyncPgConnection,
    filter: CourseRewardCandidatesFilter,
) -> Result<Vec<CourseRewardCandidate>, CourseRewardCandidatesError> {
    let mut query = reward_candidates::table.into_boxed();

    query = query.filter(reward_candidates::course_id.eq(filter.course_id));
    if let Some(student_user_id) = filter.student_user_id {
        query = query.filter(reward_candidates::student_user_id.eq(student_user_id));
    }
    if let Some(status) = filter.status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    query
        .order(reward_candidates::created_at.desc())
        .limit(filter.limit.unwrap_or(25).clamp(1, 100))
        .offset(filter.offset.unwrap_or(0).max(0))
        .select(RewardCandidate::as_select())
        .load::<RewardCandidate>(conn)
        .await
        .map(|candidates| {
            candidates
                .into_iter()
                .map(CourseRewardCandidate::from)
                .collect()
        })
        .map_err(map_course_reward_candidate_error)
}
