use bigdecimal::BigDecimal;

use crate::domain::rewards::policy::{
    RewardPaymentStrategy, RewardPolicyEventType, RewardPolicyScope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateRewardPolicyCommand {
    pub scope_type: RewardPolicyScope,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: RewardPolicyEventType,
    pub token_amount: BigDecimal,
    pub multiplier: Option<BigDecimal>,
    pub max_payout: Option<BigDecimal>,
    pub cooldown_seconds: Option<i64>,
    pub payment_strategy: RewardPaymentStrategy,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardPolicyDraft {
    pub actor_user_id: i32,
    pub scope_type: RewardPolicyScope,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub event_type: RewardPolicyEventType,
    pub token_amount: BigDecimal,
    pub multiplier: BigDecimal,
    pub max_payout: Option<BigDecimal>,
    pub cooldown_seconds: i64,
    pub payment_strategy: RewardPaymentStrategy,
    pub active: bool,
}
