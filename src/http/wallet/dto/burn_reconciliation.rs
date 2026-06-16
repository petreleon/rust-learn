use serde::Deserialize;

use crate::application::wallet::burn_tokens::TokenBurnReconciliationCommand;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenBurnReconciliationRequestDto {
    pub ethereum_address: Option<String>,
    pub platform_address: Option<String>,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub mark_failed: Option<bool>,
    pub error_message: Option<String>,
}

impl From<TokenBurnReconciliationRequestDto> for TokenBurnReconciliationCommand {
    fn from(request: TokenBurnReconciliationRequestDto) -> Self {
        Self {
            ethereum_address: request.ethereum_address,
            platform_address: request.platform_address,
            chain_id: request.chain_id,
            contract_address: request.contract_address,
            transaction_hash: request.transaction_hash,
            log_index: request.log_index,
            mark_failed: request.mark_failed,
            error_message: request.error_message,
        }
    }
}
