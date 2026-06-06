use crate::db::schema::wallet_token_deposit_intents;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

pub const WALLET_DEPOSIT_STATUS_PENDING: &str = "pending_chain_confirmation";
pub const WALLET_DEPOSIT_STATUS_CREDITED: &str = "credited";
pub const WALLET_DEPOSIT_STATUS_AMBIGUOUS: &str = "ambiguous";

#[derive(Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = wallet_token_deposit_intents)]
pub struct WalletTokenDepositIntent {
    pub id: i64,
    pub user_id: i32,
    pub wallet_id: i32,
    pub ethereum_address: String,
    pub platform_address: String,
    pub amount: BigDecimal,
    pub gas_payer: String,
    pub tax_amount: BigDecimal,
    pub status: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub event_type: Option<String>,
    pub external_transaction_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub credited_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = wallet_token_deposit_intents)]
pub struct NewWalletTokenDepositIntent {
    pub user_id: i32,
    pub wallet_id: i32,
    pub ethereum_address: String,
    pub platform_address: String,
    pub amount: BigDecimal,
    pub gas_payer: String,
    pub tax_amount: BigDecimal,
    pub status: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}
