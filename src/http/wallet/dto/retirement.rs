use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::application::wallet::retire_tokens::{WalletRetirementCommand, WalletRetirementView};

#[derive(Debug, Clone, Deserialize)]
pub struct WalletRetirementRequestDto {
    pub amount: BigDecimal,
    pub ethereum_address: String,
    pub gas_payer: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub platform_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WalletRetirementResponse {
    pub operation: String,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub internal_transaction_ids: Vec<i64>,
    pub amount: String,
    pub tax_amount: String,
    pub wallet_delta: String,
    pub gas_payer: String,
    pub ethereum_address: String,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}

impl From<WalletRetirementRequestDto> for WalletRetirementCommand {
    fn from(request: WalletRetirementRequestDto) -> Self {
        Self {
            amount: request.amount,
            ethereum_address: request.ethereum_address,
            gas_payer: request.gas_payer,
            chain_id: request.chain_id,
            contract_address: request.contract_address,
            transaction_hash: request.transaction_hash,
            log_index: request.log_index,
            platform_address: request.platform_address,
        }
    }
}

impl From<WalletRetirementView> for WalletRetirementResponse {
    fn from(view: WalletRetirementView) -> Self {
        Self {
            operation: view.operation.to_string(),
            wallet_id: view.wallet_id,
            transaction_id: view.transaction_id,
            external_transaction_id: view.external_transaction_id,
            internal_transaction_ids: view.internal_transaction_ids,
            amount: view.amount,
            tax_amount: view.tax_amount,
            wallet_delta: view.wallet_delta,
            gas_payer: view.gas_payer,
            ethereum_address: view.ethereum_address,
            wallet_provider: view.wallet_provider,
            metamask_required: view.metamask_required,
            wallet_action: view.wallet_action,
        }
    }
}
