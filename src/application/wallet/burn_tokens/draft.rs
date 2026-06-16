use bigdecimal::BigDecimal;

use crate::domain::wallet::burn::{
    TokenBurnFeePath, TokenBurnSource, TokenBurnStatus, TokenBurnerType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TokenBurnDraft {
    pub actor_user_id: i32,
    pub burner_type: TokenBurnerType,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub amount: BigDecimal,
    pub fee_amount: BigDecimal,
    pub source: TokenBurnSource,
    pub fee_path: TokenBurnFeePath,
    pub status: TokenBurnStatus,
    pub idempotency_key: String,
    pub ethereum_address: Option<String>,
    pub platform_address: Option<String>,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub deposit_intent_id: Option<i64>,
    pub permission_evidence: Option<String>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
    pub leaderboard_visible: bool,
}
