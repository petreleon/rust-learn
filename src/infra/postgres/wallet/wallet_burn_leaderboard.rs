use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Nullable, Numeric, Text, Timestamptz};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::burn_tokens::{
    TokenBurnError, TokenBurnLeaderboard, TokenBurnLeaderboardQuery, TokenBurnLeaderboardRow,
};
use crate::domain::wallet::burn::{TokenBurnLeaderboardScope, TokenBurnLeaderboardWindow};
use crate::infra::postgres::models::token_burn_leaderboard_event::NewTokenBurnLeaderboardEvent;
use crate::infra::postgres::models::token_burn_request::TokenBurnRequest;
use crate::infra::postgres::schema::token_burn_leaderboard_events;

pub(super) async fn insert_leaderboard_event(
    conn: &mut AsyncPgConnection,
    record: &TokenBurnRequest,
    now: DateTime<Utc>,
) -> Result<(), TokenBurnError> {
    if !record.leaderboard_visible {
        return Ok(());
    }

    diesel::insert_into(token_burn_leaderboard_events::table)
        .values(NewTokenBurnLeaderboardEvent {
            burn_request_id: record.id,
            burner_type: record.burner_type.clone(),
            user_id: record.user_id,
            organization_id: record.organization_id,
            amount: record.amount.clone(),
            occurred_at: now,
            visible: true,
        })
        .on_conflict_do_nothing()
        .execute(conn)
        .await
        .map_err(|error| TokenBurnError::BurnCreate(error.to_string()))?;

    Ok(())
}

pub(super) async fn load_leaderboard(
    conn: &mut AsyncPgConnection,
    query: TokenBurnLeaderboardQuery,
    window: TokenBurnLeaderboardWindow,
    scope: TokenBurnLeaderboardScope,
) -> Result<TokenBurnLeaderboard, TokenBurnError> {
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let scope_filter = match scope {
        TokenBurnLeaderboardScope::All => "",
        TokenBurnLeaderboardScope::Users => "AND burner_type = 'user'",
        TokenBurnLeaderboardScope::Organizations => "AND burner_type = 'organization'",
    };
    let sql = format!(
        "SELECT burner_type, user_id, organization_id, SUM(amount) AS total_burned, \
         COUNT(*) AS burn_count, MAX(occurred_at) AS latest_burn_at \
         FROM token_burn_leaderboard_events \
         WHERE visible = true AND occurred_at >= NOW() - INTERVAL '{} days' {} \
         GROUP BY burner_type, user_id, organization_id \
         ORDER BY SUM(amount) DESC, MAX(occurred_at) ASC, burner_type ASC, \
         COALESCE(user_id, organization_id) ASC LIMIT $1",
        window.days(),
        scope_filter
    );
    let rows = diesel::sql_query(sql)
        .bind::<BigInt, _>(limit)
        .load::<LeaderboardRow>(conn)
        .await
        .map_err(|error| TokenBurnError::LeaderboardLoad(error.to_string()))?;

    Ok(TokenBurnLeaderboard {
        window_days: window.days(),
        scope: scope_name(scope).to_string(),
        rows: rows
            .into_iter()
            .enumerate()
            .map(|(index, row)| TokenBurnLeaderboardRow {
                rank: index as i64 + 1,
                burner_type: row.burner_type,
                user_id: row.user_id,
                organization_id: row.organization_id,
                total_burned: row.total_burned.to_string(),
                burn_count: row.burn_count,
                latest_burn_at: row.latest_burn_at,
            })
            .collect(),
    })
}

fn scope_name(scope: TokenBurnLeaderboardScope) -> &'static str {
    match scope {
        TokenBurnLeaderboardScope::All => "all",
        TokenBurnLeaderboardScope::Users => "users",
        TokenBurnLeaderboardScope::Organizations => "organizations",
    }
}

#[derive(QueryableByName)]
struct LeaderboardRow {
    #[diesel(sql_type = Text)]
    burner_type: String,
    #[diesel(sql_type = Nullable<Integer>)]
    user_id: Option<i32>,
    #[diesel(sql_type = Nullable<Integer>)]
    organization_id: Option<i32>,
    #[diesel(sql_type = Numeric)]
    total_burned: BigDecimal,
    #[diesel(sql_type = BigInt)]
    burn_count: i64,
    #[diesel(sql_type = Timestamptz)]
    latest_burn_at: DateTime<Utc>,
}
