pub use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexOutput as WalletDepositCreditResult,
};
use crate::application::wallet::index_deposit::WalletDepositIndexError;
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{users, wallet_token_deposit_intents, wallets};
use crate::infra::postgres::wallet::wallet_deposit_index_store::PostgresWalletDepositIndexStore;
use crate::models::wallet::{NewWallet, Wallet};
use crate::models::wallet_token_deposit_intent::{
    NewWalletTokenDepositIntent, WalletTokenDepositIntent, WALLET_DEPOSIT_STATUS_PENDING,
};
use crate::repositories::persistent_state_repository::{
    get_persistent_state, set_persistent_state,
};
use crate::repositories::platform_repository::user_permission_platform_request;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::{env, str::FromStr};

pub const TOKEN_TRANSFER_OPERATION_DEPOSIT: &str = "deposit";
pub const TOKEN_TRANSFER_OPERATION_RETIRE: &str = "retire";
pub const TOKEN_TRANSFER_GAS_PAYER_USER: &str = "user";
pub const TOKEN_TRANSFER_GAS_PAYER_PLATFORM: &str = "platform";
pub const TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK: &str = "metamask";
pub const TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM: &str = "platform";

const TOKEN_DEPOSIT_TAX_KEY: &str = "wallet.deposit_tax_tokens";
const TOKEN_RETIRE_TAX_KEY: &str = "wallet.retire_tax_tokens";
const TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER: &str = "metamask_transfer";
const TOKEN_TRANSFER_ACTION_METAMASK_PERMIT_SIGNATURE: &str = "metamask_permit_signature";
const TOKEN_TRANSFER_ACTION_METAMASK_PRESIGNED_TRANSFER: &str = "metamask_presigned_transfer";
const TOKEN_TRANSFER_ACTION_PLATFORM_TRANSFER: &str = "platform_transfer";

#[derive(Debug)]
pub struct LinkedWallet {
    pub wallet: Wallet,
    pub created: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetWalletTokenTaxRequest {
    pub tax_amount: BigDecimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTokenTransferRequest {
    pub amount: BigDecimal,
    pub ethereum_address: String,
    pub gas_payer: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub platform_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WalletTokenTaxResponse {
    pub operation: String,
    pub tax_amount: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WalletTokenTaxSettingsResponse {
    pub deposit: WalletTokenTaxResponse,
    pub retire: WalletTokenTaxResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WalletTokenDepositIntentResponse {
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

#[derive(Debug, PartialEq)]
pub enum WalletTokenTransferError {
    PermissionDenied(String),
    KycRequired,
    InvalidInput(String),
    InsufficientFunds,
    Database(String),
}

impl From<WalletDepositIndexError> for WalletTokenTransferError {
    fn from(error: WalletDepositIndexError) -> Self {
        match error {
            WalletDepositIndexError::InvalidInput(message) => {
                WalletTokenTransferError::InvalidInput(message)
            }
            WalletDepositIndexError::InsufficientFunds => {
                WalletTokenTransferError::InsufficientFunds
            }
            WalletDepositIndexError::Connection(message)
            | WalletDepositIndexError::Database(message) => {
                WalletTokenTransferError::Database(message)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletTokenOperation {
    Deposit,
    Retire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WalletTokenGasPayer {
    User,
    Platform,
}
