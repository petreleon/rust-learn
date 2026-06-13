use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct RewardCompensationRecordOutput {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: BigDecimal,
    pub reason: String,
    pub idempotency_key: String,
    pub created_by_user_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardCompensationWalletOutput {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub value: BigDecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardCompensationOutput {
    pub record: RewardCompensationRecordOutput,
    pub wallet: RewardCompensationWalletOutput,
    pub created: bool,
}
