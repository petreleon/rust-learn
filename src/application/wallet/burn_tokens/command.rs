use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenBurnCommand {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBurnLeaderboardQuery {
    pub window: String,
    pub scope: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBurnReconciliationCommand {
    pub ethereum_address: Option<String>,
    pub platform_address: Option<String>,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub mark_failed: Option<bool>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBurnSubject {
    OwnUser,
    Organization(i32),
}
