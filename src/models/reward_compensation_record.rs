use crate::db::schema::reward_compensation_records;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = reward_compensation_records)]
pub struct RewardCompensationRecord {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: BigDecimal,
    pub reason: String,
    pub idempotency_key: String,
    pub created_by_user_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reward_compensation_records)]
pub struct NewRewardCompensationRecord {
    pub reward_candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: BigDecimal,
    pub reason: String,
    pub idempotency_key: String,
    pub created_by_user_id: i32,
}
