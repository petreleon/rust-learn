use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardWalletCreditOutput {
    pub candidate_id: i64,
    pub wallet_id: i32,
    pub credit_record_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub amount: BigDecimal,
    pub credited: bool,
}
