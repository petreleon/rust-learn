use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardWalletCreditNotificationOutput {
    pub candidate_id: i64,
    pub wallet_id: i32,
    pub notification_id: Option<i64>,
    pub transaction_id: i64,
    pub amount: BigDecimal,
    pub notified: bool,
}
