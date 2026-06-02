use crate::db::schema::reward_execution_jobs;
use crate::models::reward_execution_job::{
    NewRewardExecutionJob, RewardExecutionJob, REWARD_EXECUTION_STATUS_QUEUED,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn enqueue_reward_execution_job(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> QueryResult<RewardExecutionJob> {
    let new_job = NewRewardExecutionJob {
        reward_candidate_id,
        status: REWARD_EXECUTION_STATUS_QUEUED.to_string(),
    };

    diesel::insert_into(reward_execution_jobs::table)
        .values(&new_job)
        .on_conflict(reward_execution_jobs::reward_candidate_id)
        .do_update()
        .set(reward_execution_jobs::updated_at.eq(chrono::Utc::now()))
        .get_result(conn)
        .await
}

pub async fn find_job_by_candidate(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> QueryResult<Option<RewardExecutionJob>> {
    reward_execution_jobs::table
        .filter(reward_execution_jobs::reward_candidate_id.eq(reward_candidate_id))
        .first::<RewardExecutionJob>(conn)
        .await
        .optional()
}
