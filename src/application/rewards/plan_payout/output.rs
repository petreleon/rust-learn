use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutPlan {
    pub candidate_id: i64,
    pub policy_id: i64,
    pub amount: BigDecimal,
    pub payment_strategy: String,
    pub payout_method: String,
    pub requires_token_confirmation: bool,
}
