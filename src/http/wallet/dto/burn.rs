use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::wallet::burn_tokens::{
    TokenBurnCommand, TokenBurnLeaderboard, TokenBurnLeaderboardQuery, TokenBurnLeaderboardRow,
    TokenBurnView,
};

#[derive(Debug, Clone, Deserialize)]
pub struct TokenBurnRequestDto {
    pub amount: BigDecimal,
    pub source: String,
    pub fee_path: String,
    pub idempotency_key: String,
    pub ethereum_address: Option<String>,
    pub platform_address: Option<String>,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub deposit_intent_id: Option<i64>,
    pub leaderboard_visible: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenBurnLeaderboardQueryDto {
    pub window: String,
    pub scope: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TokenBurnResponse {
    pub id: i64,
    pub actor_user_id: i32,
    pub burner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub wallet_id: Option<i32>,
    pub source: String,
    pub fee_path: String,
    pub status: String,
    pub amount: String,
    pub fee_amount: String,
    pub idempotency_key: String,
    pub deposit_intent_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub external_transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub permission_evidence: Option<String>,
    pub wallet_action: String,
    pub metamask_required: bool,
    pub leaderboard_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TokenBurnLeaderboardResponse {
    pub window_days: i64,
    pub scope: String,
    pub rows: Vec<TokenBurnLeaderboardRowResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TokenBurnLeaderboardRowResponse {
    pub rank: i64,
    pub burner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub total_burned: String,
    pub burn_count: i64,
    pub latest_burn_at: DateTime<Utc>,
}

impl From<TokenBurnRequestDto> for TokenBurnCommand {
    fn from(request: TokenBurnRequestDto) -> Self {
        Self {
            amount: request.amount,
            source: request.source,
            fee_path: request.fee_path,
            idempotency_key: request.idempotency_key,
            ethereum_address: request.ethereum_address,
            platform_address: request.platform_address,
            chain_id: request.chain_id,
            contract_address: request.contract_address,
            transaction_hash: request.transaction_hash,
            log_index: request.log_index,
            deposit_intent_id: request.deposit_intent_id,
            leaderboard_visible: request.leaderboard_visible,
        }
    }
}

impl From<TokenBurnLeaderboardQueryDto> for TokenBurnLeaderboardQuery {
    fn from(query: TokenBurnLeaderboardQueryDto) -> Self {
        Self {
            window: query.window,
            scope: query.scope,
            limit: query.limit,
        }
    }
}

impl From<TokenBurnView> for TokenBurnResponse {
    fn from(view: TokenBurnView) -> Self {
        Self {
            id: view.id,
            actor_user_id: view.actor_user_id,
            burner_type: view.burner_type,
            user_id: view.user_id,
            organization_id: view.organization_id,
            wallet_id: view.wallet_id,
            source: view.source,
            fee_path: view.fee_path,
            status: view.status,
            amount: view.amount,
            fee_amount: view.fee_amount,
            idempotency_key: view.idempotency_key,
            deposit_intent_id: view.deposit_intent_id,
            transaction_id: view.transaction_id,
            external_transaction_id: view.external_transaction_id,
            internal_transaction_id: view.internal_transaction_id,
            permission_evidence: view.permission_evidence,
            wallet_action: view.wallet_action,
            metamask_required: view.metamask_required,
            leaderboard_visible: view.leaderboard_visible,
            created_at: view.created_at,
            updated_at: view.updated_at,
        }
    }
}

impl From<TokenBurnLeaderboard> for TokenBurnLeaderboardResponse {
    fn from(leaderboard: TokenBurnLeaderboard) -> Self {
        Self {
            window_days: leaderboard.window_days,
            scope: leaderboard.scope,
            rows: leaderboard
                .rows
                .into_iter()
                .map(TokenBurnLeaderboardRowResponse::from)
                .collect(),
        }
    }
}

impl From<TokenBurnLeaderboardRow> for TokenBurnLeaderboardRowResponse {
    fn from(row: TokenBurnLeaderboardRow) -> Self {
        Self {
            rank: row.rank,
            burner_type: row.burner_type,
            user_id: row.user_id,
            organization_id: row.organization_id,
            total_burned: row.total_burned,
            burn_count: row.burn_count,
            latest_burn_at: row.latest_burn_at,
        }
    }
}
