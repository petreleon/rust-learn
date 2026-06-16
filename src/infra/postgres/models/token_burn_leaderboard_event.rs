use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::token_burn_leaderboard_events;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = token_burn_leaderboard_events)]
pub struct TokenBurnLeaderboardEvent {
    pub id: i64,
    pub burn_request_id: i64,
    pub burner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub amount: BigDecimal,
    pub occurred_at: DateTime<Utc>,
    pub visible: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = token_burn_leaderboard_events)]
pub struct NewTokenBurnLeaderboardEvent {
    pub burn_request_id: i64,
    pub burner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub amount: BigDecimal,
    pub occurred_at: DateTime<Utc>,
    pub visible: bool,
}
