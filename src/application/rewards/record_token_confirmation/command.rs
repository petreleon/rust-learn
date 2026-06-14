use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardTokenConfirmationCommand {
    pub chain_id: i64,
    pub contract_address: String,
    pub transaction_hash: String,
    pub log_index: i64,
    pub event_type: String,
    pub from_address: Option<String>,
    pub to_address: String,
    pub amount: BigDecimal,
}
