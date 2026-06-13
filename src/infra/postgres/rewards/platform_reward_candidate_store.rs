use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_platform_candidates::store::PlatformRewardCandidateStore;
use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidateCourseSummary, PlatformRewardCandidateRecord,
    PlatformRewardCandidateUserSummary, PlatformRewardCandidatesError,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, users};
use crate::infra::postgres::rewards::platform_reward_candidate_mappers::map_platform_reward_candidate_error;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_candidate_repository::{self, RewardCandidateFilter};

pub struct PostgresPlatformRewardCandidateStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresPlatformRewardCandidateStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformRewardCandidateStore for PostgresPlatformRewardCandidateStore<'_> {
    fn can_view_reward_audit(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRewardCandidatesError>> {
        self.platform_permission(actor_user_id, Permissions::VIEW_REWARD_AUDIT)
    }

    fn can_approve_reward_amount(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRewardCandidatesError>> {
        self.platform_permission(actor_user_id, Permissions::APPROVE_REWARD_AMOUNT)
    }

    fn list_candidate_records(
        &mut self,
        status: Option<String>,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardCandidateRecord>, PlatformRewardCandidatesError>>
    {
        async move {
            reward_candidate_repository::list_candidates(
                self.conn,
                RewardCandidateFilter {
                    course_id: None,
                    student_user_id: None,
                    status,
                    limit: None,
                    offset: None,
                },
            )
            .await
            .map(|records| {
                records
                    .into_iter()
                    .map(PlatformRewardCandidateRecord::from)
                    .collect()
            })
            .map_err(map_platform_reward_candidate_error)
        }
        .boxed()
    }

    fn count_candidate_records(
        &mut self,
        status: Option<String>,
    ) -> BoxFuture<'_, Result<i64, PlatformRewardCandidatesError>> {
        async move {
            reward_candidate_repository::count_candidates(
                self.conn,
                RewardCandidateFilter {
                    course_id: None,
                    student_user_id: None,
                    status,
                    limit: None,
                    offset: None,
                },
            )
            .await
            .map_err(map_platform_reward_candidate_error)
        }
        .boxed()
    }

    fn load_user_summaries(
        &mut self,
        user_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<Vec<PlatformRewardCandidateUserSummary>, PlatformRewardCandidatesError>>
    {
        async move {
            if user_ids.is_empty() {
                return Ok(Vec::new());
            }

            users::table
                .filter(users::id.eq_any(user_ids))
                .select((users::id, users::name, users::email))
                .load::<(i32, String, String)>(self.conn)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|(id, name, email)| PlatformRewardCandidateUserSummary {
                            id,
                            name,
                            email,
                        })
                        .collect()
                })
                .map_err(map_platform_reward_candidate_error)
        }
        .boxed()
    }

    fn load_course_summaries(
        &mut self,
        course_ids: Vec<i32>,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRewardCandidateCourseSummary>, PlatformRewardCandidatesError>,
    > {
        async move {
            if course_ids.is_empty() {
                return Ok(Vec::new());
            }

            courses::table
                .filter(courses::id.eq_any(course_ids))
                .select((courses::id, courses::title))
                .load::<(i32, String)>(self.conn)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|(id, title)| PlatformRewardCandidateCourseSummary { id, title })
                        .collect()
                })
                .map_err(map_platform_reward_candidate_error)
        }
        .boxed()
    }
}

impl PostgresPlatformRewardCandidateStore<'_> {
    fn platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: Permissions,
    ) -> BoxFuture<'_, Result<bool, PlatformRewardCandidatesError>> {
        async move {
            user_permission_platform_request(self.conn, actor_user_id, &permission.to_string())
                .await
                .map_err(map_platform_reward_candidate_error)
        }
        .boxed()
    }
}
