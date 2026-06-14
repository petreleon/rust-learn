use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardAmountDecisionCommand {
    pub status: String,
    pub approved_amount: Option<BigDecimal>,
    pub decision_reason: Option<String>,
}
