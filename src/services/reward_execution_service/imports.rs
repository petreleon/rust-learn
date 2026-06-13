use crate::config::constants::permissions::Permissions;
use crate::db::schema::{transactions_external_transactions, transactions_internal_transactions};
pub use crate::application::rewards::credit_wallet::RewardWalletCreditOutput as RewardWalletCreditResult;
use crate::application::rewards::credit_wallet::RewardWalletCreditError;
pub use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationOutput as RewardWalletCreditNotificationResult;
use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationError;
pub use crate::application::rewards::plan_payout::RewardPayoutPlan;
use crate::application::rewards::plan_payout::RewardPayoutPlanError;
pub use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmationCommand as RewardTokenConfirmationRequest,
    RewardTokenConfirmationOutput as RewardTokenConfirmationResult,
};
use crate::application::rewards::record_token_confirmation::RewardTokenConfirmationError;
pub use crate::domain::rewards::payout::{
    REWARD_PAYOUT_METHOD_MINT, REWARD_PAYOUT_METHOD_OFF_CHAIN,
    REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER, REWARD_PAYOUT_METHOD_TREASURY_TRANSFER,
};
pub use crate::domain::rewards::wallet_credit::REWARD_TRANSACTION_TYPE_WALLET_CREDIT;
use crate::infra::postgres::rewards::reward_payout_plan_store::PostgresRewardPayoutPlanStore;
use crate::infra::postgres::rewards::reward_token_confirmation_store::PostgresRewardTokenConfirmationStore;
use crate::infra::postgres::rewards::reward_wallet_credit_notification_store::PostgresRewardWalletCreditNotificationStore;
use crate::infra::postgres::rewards::reward_wallet_credit_store::PostgresRewardWalletCreditStore;
use crate::models::reward_audit_event::{NewRewardAuditEvent, REWARD_AUDIT_EVENT_RECONCILED};
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_COMPLETED,
    REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED, REWARD_STATUS_TOKEN_CONFIRMED,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::transaction::{
    NewTransactionExternalTransactionLink, NewTransactionInternalTransactionLink,
};
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_audit_event_repository;
use crate::repositories::reward_candidate_repository;
use crate::repositories::reward_payout_record_repository;
use crate::repositories::reward_wallet_credit_record_repository;
use diesel::prelude::*;
use diesel_async::AsyncConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Debug, Clone, PartialEq)]
pub struct RewardReconciliationResult {
    pub candidate_id: i64,
    pub wallet_credit_created: bool,
    pub notification_created: bool,
    pub external_transaction_link_repaired: bool,
    pub internal_transaction_link_repaired: bool,
    pub final_status: String,
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

impl From<RewardPayoutPlanError> for RewardExecutionError {
    fn from(error: RewardPayoutPlanError) -> Self {
        match error {
            RewardPayoutPlanError::PermissionDenied(permission) => {
                RewardExecutionError::PermissionDenied(permission)
            }
            RewardPayoutPlanError::InvalidStatus(message) => {
                RewardExecutionError::InvalidStatus(message)
            }
            RewardPayoutPlanError::InvalidInput(message) => {
                RewardExecutionError::InvalidInput(message)
            }
            RewardPayoutPlanError::NoActivePolicy => RewardExecutionError::NoActivePolicy,
            RewardPayoutPlanError::Connection(message)
            | RewardPayoutPlanError::Database(message) => {
                RewardExecutionError::Database(message)
            }
        }
    }
}

impl From<RewardTokenConfirmationError> for RewardExecutionError {
    fn from(error: RewardTokenConfirmationError) -> Self {
        match error {
            RewardTokenConfirmationError::PermissionDenied(permission) => {
                RewardExecutionError::PermissionDenied(permission)
            }
            RewardTokenConfirmationError::InvalidStatus(message) => {
                RewardExecutionError::InvalidStatus(message)
            }
            RewardTokenConfirmationError::InvalidInput(message) => {
                RewardExecutionError::InvalidInput(message)
            }
            RewardTokenConfirmationError::NotFound => RewardExecutionError::NoActivePolicy,
            RewardTokenConfirmationError::Connection(message)
            | RewardTokenConfirmationError::Database(message) => {
                RewardExecutionError::Database(message)
            }
        }
    }
}

impl From<RewardWalletCreditError> for RewardExecutionError {
    fn from(error: RewardWalletCreditError) -> Self {
        match error {
            RewardWalletCreditError::PermissionDenied(permission) => {
                RewardExecutionError::PermissionDenied(permission)
            }
            RewardWalletCreditError::InvalidStatus(message) => {
                RewardExecutionError::InvalidStatus(message)
            }
            RewardWalletCreditError::InvalidInput(message) => {
                RewardExecutionError::InvalidInput(message)
            }
            RewardWalletCreditError::NoActivePolicy | RewardWalletCreditError::NotFound => {
                RewardExecutionError::NoActivePolicy
            }
            RewardWalletCreditError::Connection(message) | RewardWalletCreditError::Database(message) => {
                RewardExecutionError::Database(message)
            }
        }
    }
}

impl From<RewardWalletCreditNotificationError> for RewardExecutionError {
    fn from(error: RewardWalletCreditNotificationError) -> Self {
        match error {
            RewardWalletCreditNotificationError::PermissionDenied(permission) => {
                RewardExecutionError::PermissionDenied(permission)
            }
            RewardWalletCreditNotificationError::InvalidStatus(message) => {
                RewardExecutionError::InvalidStatus(message)
            }
            RewardWalletCreditNotificationError::InvalidInput(message) => {
                RewardExecutionError::InvalidInput(message)
            }
            RewardWalletCreditNotificationError::NotFound => RewardExecutionError::NoActivePolicy,
            RewardWalletCreditNotificationError::Connection(message)
            | RewardWalletCreditNotificationError::Database(message) => {
                RewardExecutionError::Database(message)
            }
        }
    }
}
