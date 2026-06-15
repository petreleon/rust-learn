use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use crate::application::wallet::create_deposit_intent::{
    WalletDepositIntentCommand, WalletDepositIntentView,
};

#[derive(Debug, Clone, Deserialize)]
pub struct WalletDepositIntentRequestDto {
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
pub struct WalletDepositIntentResponse {
    pub operation: String,
    pub id: i64,
    pub status: String,
    pub wallet_id: i32,
    pub amount: String,
    pub tax_amount: String,
    pub wallet_delta_on_confirmation: String,
    pub gas_payer: String,
    pub ethereum_address: String,
    pub platform_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}

impl From<WalletDepositIntentRequestDto> for WalletDepositIntentCommand {
    fn from(request: WalletDepositIntentRequestDto) -> Self {
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

impl From<WalletDepositIntentView> for WalletDepositIntentResponse {
    fn from(view: WalletDepositIntentView) -> Self {
        Self {
            operation: view.operation.to_string(),
            id: view.id,
            status: view.status.as_str().to_string(),
            wallet_id: view.wallet_id,
            amount: view.amount,
            tax_amount: view.tax_amount,
            wallet_delta_on_confirmation: view.wallet_delta_on_confirmation,
            gas_payer: view.gas_payer,
            ethereum_address: view.ethereum_address,
            platform_address: view.platform_address,
            chain_id: view.chain_id,
            contract_address: view.contract_address,
            transaction_hash: view.transaction_hash,
            log_index: view.log_index,
            wallet_provider: view.wallet_provider,
            metamask_required: view.metamask_required,
            wallet_action: view.wallet_action,
        }
    }
}
