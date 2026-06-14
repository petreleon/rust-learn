use crate::application::rewards::credit_wallet::RewardWalletCreditError;
pub use crate::application::rewards::credit_wallet::RewardWalletCreditOutput as RewardWalletCreditResult;
use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationError;
pub use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationOutput as RewardWalletCreditNotificationResult;
pub use crate::application::rewards::plan_payout::RewardPayoutPlan;
use crate::application::rewards::plan_payout::RewardPayoutPlanError;
use crate::application::rewards::reconcile_candidate::RewardReconciliationError;
pub use crate::application::rewards::reconcile_candidate::RewardReconciliationOutput as RewardReconciliationResult;
use crate::application::rewards::record_token_confirmation::RewardTokenConfirmationError;
pub use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmationCommand as RewardTokenConfirmationRequest,
    RewardTokenConfirmationOutput as RewardTokenConfirmationResult,
};
pub use crate::domain::rewards::payout::{
    REWARD_PAYOUT_METHOD_MINT, REWARD_PAYOUT_METHOD_OFF_CHAIN,
    REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER, REWARD_PAYOUT_METHOD_TREASURY_TRANSFER,
};
pub use crate::domain::rewards::wallet_credit::REWARD_TRANSACTION_TYPE_WALLET_CREDIT;

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
            | RewardPayoutPlanError::Database(message) => RewardExecutionError::Database(message),
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
            RewardWalletCreditError::Connection(message)
            | RewardWalletCreditError::Database(message) => RewardExecutionError::Database(message),
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

impl From<RewardReconciliationError> for RewardExecutionError {
    fn from(error: RewardReconciliationError) -> Self {
        match error {
            RewardReconciliationError::PermissionDenied(permission) => {
                RewardExecutionError::PermissionDenied(permission)
            }
            RewardReconciliationError::InvalidStatus(message) => {
                RewardExecutionError::InvalidStatus(message)
            }
            RewardReconciliationError::InvalidInput(message) => {
                RewardExecutionError::InvalidInput(message)
            }
            RewardReconciliationError::NoActivePolicy | RewardReconciliationError::NotFound => {
                RewardExecutionError::NoActivePolicy
            }
            RewardReconciliationError::Connection(message)
            | RewardReconciliationError::Database(message) => {
                RewardExecutionError::Database(message)
            }
        }
    }
}
