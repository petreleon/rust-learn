use crate::infra::postgres::schema::reward_wallet_credit_records;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = reward_wallet_credit_records)]
pub struct RewardWalletCreditRecord {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reward_wallet_credit_records)]
pub struct NewRewardWalletCreditRecord {
    pub reward_candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
}
