use crate::infra::postgres::schema::reward_payout_records;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = reward_payout_records)]
pub struct RewardPayoutRecord {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reward_payout_records)]
pub struct NewRewardPayoutRecord {
    pub reward_candidate_id: i64,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
}
