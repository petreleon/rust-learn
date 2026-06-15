use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_execution_jobs;
use crate::domain::rewards::execution::RewardExecutionJobStatus;
use crate::infra::postgres::models::reward_execution_job::{
    NewRewardExecutionJob, RewardExecutionJob,
};

pub async fn enqueue_reward_execution_job(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> QueryResult<RewardExecutionJob> {
    let new_job = NewRewardExecutionJob {
        reward_candidate_id,
        status: RewardExecutionJobStatus::Queued.as_str().to_string(),
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
