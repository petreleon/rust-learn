use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, courses_organizations, external_transactions, internal_transactions,
    reward_candidates, reward_policies, transactions, transactions_external_transactions,
    transactions_internal_transactions, wallets,
};
use crate::models::reward_audit_event::{
    NewRewardAuditEvent, REWARD_AUDIT_EVENT_RECONCILED, REWARD_AUDIT_EVENT_TOKEN_CONFIRMED,
    REWARD_AUDIT_EVENT_WALLET_CREDITED, REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED,
};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_COMPLETED,
    REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED, REWARD_STATUS_TOKEN_CONFIRMED,
    REWARD_STATUS_TOKEN_PENDING, REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_payout_record::NewRewardPayoutRecord;
use crate::models::reward_policy::{
    RewardPolicy, REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::models::reward_wallet_credit_record::NewRewardWalletCreditRecord;
use crate::models::transaction::{
    ExternalTransaction, NewExternalTransaction, NewInternalTransaction, NewTransaction,
    NewTransactionExternalTransactionLink, NewTransactionInternalTransactionLink,
};
use crate::models::wallet::Wallet;
use crate::repositories::persistent_state_repository::get_persistent_state;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_audit_event_repository;
use crate::repositories::reward_candidate_repository;
use crate::repositories::reward_payout_record_repository;
use crate::repositories::reward_wallet_credit_record_repository;
use crate::services::wallet_service;
use crate::utils::notifications::{create_notification, reward_wallet_credit_notification};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::AsyncConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub const REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER: &str = "presigner_transfer";
pub const REWARD_PAYOUT_METHOD_TREASURY_TRANSFER: &str = "treasury_transfer";
pub const REWARD_PAYOUT_METHOD_MINT: &str = "mint";
pub const REWARD_PAYOUT_METHOD_OFF_CHAIN: &str = "off_chain";
pub const REWARD_TRANSACTION_TYPE_WALLET_CREDIT: &str = "reward_wallet_credit";

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutPlan {
    pub candidate_id: i64,
    pub policy_id: i64,
    pub amount: BigDecimal,
    pub payment_strategy: String,
    pub payout_method: String,
    pub requires_token_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardWalletCreditResult {
    pub candidate_id: i64,
    pub wallet_id: i32,
    pub credit_record_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub internal_transaction_id: Option<i64>,
    pub amount: BigDecimal,
    pub credited: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardWalletCreditNotificationResult {
    pub candidate_id: i64,
    pub wallet_id: i32,
    pub notification_id: Option<i64>,
    pub transaction_id: i64,
    pub amount: BigDecimal,
    pub notified: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardReconciliationResult {
    pub candidate_id: i64,
    pub wallet_credit_created: bool,
    pub notification_created: bool,
    pub external_transaction_link_repaired: bool,
    pub internal_transaction_link_repaired: bool,
    pub final_status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardTokenConfirmationRequest {
    pub chain_id: i64,
    pub contract_address: String,
    pub transaction_hash: String,
    pub log_index: i64,
    pub event_type: String,
    pub from_address: Option<String>,
    pub to_address: String,
    pub amount: BigDecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardTokenConfirmationResult {
    pub candidate_id: i64,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub payout_record_id: i64,
    pub inserted_external_transaction: bool,
}

#[derive(Debug, PartialEq)]
pub enum RewardExecutionError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidInput(String),
    NoActivePolicy,
    Database(String),
}

impl From<diesel::result::Error> for RewardExecutionError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardExecutionError::NoActivePolicy,
            other => RewardExecutionError::Database(other.to_string()),
        }
    }
}
