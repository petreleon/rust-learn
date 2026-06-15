use crate::db::schema::reward_policies;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = reward_policies)]
pub struct RewardPolicy {
    pub id: i64,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: String,
    pub version: i32,
    pub token_amount: BigDecimal,
    pub multiplier: BigDecimal,
    pub max_payout: Option<BigDecimal>,
    pub cooldown_seconds: i64,
    pub payment_strategy: String,
    pub active: bool,
    pub created_by_user_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reward_policies)]
pub struct NewRewardPolicy {
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: String,
    pub version: i32,
    pub token_amount: BigDecimal,
    pub multiplier: BigDecimal,
    pub max_payout: Option<BigDecimal>,
    pub cooldown_seconds: i64,
    pub payment_strategy: String,
    pub active: bool,
    pub created_by_user_id: Option<i32>,
}
