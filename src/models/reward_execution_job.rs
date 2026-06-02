use crate::db::schema::reward_execution_jobs;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

pub const REWARD_EXECUTION_STATUS_QUEUED: &str = "queued";

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = reward_execution_jobs)]
pub struct RewardExecutionJob {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub status: String,
    pub attempts: i32,
    pub run_after: DateTime<Utc>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reward_execution_jobs)]
pub struct NewRewardExecutionJob {
    pub reward_candidate_id: i64,
    pub status: String,
}
