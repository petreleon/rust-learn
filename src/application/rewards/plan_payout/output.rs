use bigdecimal::BigDecimal;

use crate::domain::rewards::payout::RewardPayoutMethod;
use crate::domain::rewards::policy::RewardPaymentStrategy;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutPlan {
    pub candidate_id: i64,
    pub policy_id: i64,
    pub amount: BigDecimal,
    pub payment_strategy: RewardPaymentStrategy,
    pub payout_method: RewardPayoutMethod,
    pub requires_token_confirmation: bool,
}
