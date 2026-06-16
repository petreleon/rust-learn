use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::token_burn_requests;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = token_burn_requests)]
pub struct TokenBurnRequest {
    pub id: i64,
    pub actor_user_id: i32,
    pub burner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub wallet_id: Option<i32>,
    pub source: String,
    pub fee_path: String,
    pub status: String,
    pub amount: BigDecimal,
    pub fee_amount: BigDecimal,
    pub idempotency_key: String,
    pub deposit_intent_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub external_transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub permission_evidence: Option<String>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
    pub leaderboard_visible: bool,
    pub last_error: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub ledger_recorded_at: Option<DateTime<Utc>>,
    pub leaderboard_indexed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = token_burn_requests)]
pub struct NewTokenBurnRequest {
    pub actor_user_id: i32,
    pub burner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub wallet_id: Option<i32>,
    pub source: String,
    pub fee_path: String,
    pub status: String,
    pub amount: BigDecimal,
    pub fee_amount: BigDecimal,
    pub idempotency_key: String,
    pub deposit_intent_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub external_transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub permission_evidence: Option<String>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
    pub leaderboard_visible: bool,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub ledger_recorded_at: Option<DateTime<Utc>>,
    pub leaderboard_indexed_at: Option<DateTime<Utc>>,
}
