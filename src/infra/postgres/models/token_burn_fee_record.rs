use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::token_burn_fee_records;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = token_burn_fee_records)]
pub struct TokenBurnFeeRecord {
    pub id: i64,
    pub burn_request_id: i64,
    pub actor_user_id: i32,
    pub fee_path: String,
    pub amount: BigDecimal,
    pub transaction_id: Option<i64>,
    pub deposit_intent_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = token_burn_fee_records)]
pub struct NewTokenBurnFeeRecord {
    pub burn_request_id: i64,
    pub actor_user_id: i32,
    pub fee_path: String,
    pub amount: BigDecimal,
    pub transaction_id: Option<i64>,
    pub deposit_intent_id: Option<i64>,
}
