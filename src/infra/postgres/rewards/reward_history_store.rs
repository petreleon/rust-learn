use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_reward_history::{
    StudentRewardCandidateRecord, StudentRewardHistoryError, StudentRewardHistoryFilter,
    StudentRewardTokenTransaction, StudentRewardWalletCredit,
};
use crate::application::rewards::ports::StudentRewardHistoryStore;
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_history_financials;
use crate::infra::postgres::rewards::reward_history_mappers::{
    candidate_record, map_reward_history_error,
};
use crate::infra::postgres::schema::{courses, reward_candidates};

pub struct PostgresStudentRewardHistoryStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresStudentRewardHistoryStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl StudentRewardHistoryStore for PostgresStudentRewardHistoryStore<'_> {
    fn list_student_reward_candidates(
        &mut self,
        filter: StudentRewardHistoryFilter,
    ) -> BoxFuture<'_, Result<Vec<StudentRewardCandidateRecord>, StudentRewardHistoryError>> {
        async move { list_candidates(self.conn, filter).await }.boxed()
    }

    fn can_view_course_reward_status(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, StudentRewardHistoryError>> {
        async move {
            reward_authorization_access::can_view_course_reward_status(
                self.conn,
                actor_user_id,
                course_id,
            )
            .await
            .map_err(|error| StudentRewardHistoryError::Database(error.to_string()))
        }
        .boxed()
    }

    fn load_wallet_credit(
        &mut self,
        reward_candidate_id: i64,
    ) -> BoxFuture<'_, Result<Option<StudentRewardWalletCredit>, StudentRewardHistoryError>> {
        async move {
            reward_history_financials::load_wallet_credit(self.conn, reward_candidate_id).await
        }
        .boxed()
    }

    fn load_token_transaction(
        &mut self,
        reward_candidate_id: i64,
    ) -> BoxFuture<'_, Result<Option<StudentRewardTokenTransaction>, StudentRewardHistoryError>>
    {
        async move {
            reward_history_financials::load_token_transaction(self.conn, reward_candidate_id).await
        }
        .boxed()
    }
}

async fn list_candidates(
    conn: &mut AsyncPgConnection,
    filter: StudentRewardHistoryFilter,
) -> Result<Vec<StudentRewardCandidateRecord>, StudentRewardHistoryError> {
    let mut query = reward_candidates::table
        .inner_join(courses::table.on(reward_candidates::course_id.eq(courses::id)))
        .filter(reward_candidates::student_user_id.eq(filter.student_user_id))
        .into_boxed();

    if let Some(course_id) = filter.course_id {
        query = query.filter(reward_candidates::course_id.eq(course_id));
    }
    if let Some(status) = filter.status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    let rows = query
        .select((RewardCandidate::as_select(), courses::title))
        .order(reward_candidates::created_at.desc())
        .limit(filter.limit)
        .offset(filter.offset)
        .load::<(RewardCandidate, String)>(conn)
        .await
        .map_err(map_reward_history_error)?;

    rows.into_iter()
        .map(|(candidate, title)| candidate_record(candidate, title))
        .collect()
}
