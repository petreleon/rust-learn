use bigdecimal::BigDecimal;

#[derive(Debug, Clone, PartialEq)]
pub struct WalletDepositIntentCommand {
    pub amount: BigDecimal,
    pub ethereum_address: String,
    pub gas_payer: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub platform_address: Option<String>,
}
