use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBurnView {
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
    pub last_error: Option<String>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
    pub leaderboard_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBurnLeaderboard {
    pub window_days: i64,
    pub scope: String,
    pub rows: Vec<TokenBurnLeaderboardRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBurnLeaderboardRow {
    pub rank: i64,
    pub burner_type: String,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub total_burned: String,
    pub burn_count: i64,
    pub latest_burn_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationTokenBurnPermissions {
    pub organization_id: i32,
    pub required_permission: String,
    pub can_burn: bool,
    pub kyc_verified: bool,
    pub can_request_burn: bool,
}
