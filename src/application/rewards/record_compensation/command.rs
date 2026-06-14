use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct RecordRewardCompensationCommand {
    pub reward_candidate_id: i64,
    pub amount: BigDecimal,
    pub reason: String,
    pub idempotency_key: String,
}
