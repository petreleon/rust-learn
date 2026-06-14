use crate::application::rewards::credit_wallet::RewardWalletCreditError;
use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationError;
use crate::application::rewards::reconcile_candidate::RewardReconciliationError;

pub(super) enum RewardReconciliationTransactionError {
    Application(RewardReconciliationError),
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for RewardReconciliationTransactionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl From<RewardReconciliationError> for RewardReconciliationTransactionError {
    fn from(error: RewardReconciliationError) -> Self {
        Self::Application(error)
    }
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> RewardReconciliationError {
    match error {
        diesel::result::Error::NotFound => RewardReconciliationError::NotFound,
        other => RewardReconciliationError::Database(other.to_string()),
    }
}

pub(super) fn map_wallet_credit_error(error: RewardWalletCreditError) -> RewardReconciliationError {
    match error {
        RewardWalletCreditError::PermissionDenied(permission) => {
            RewardReconciliationError::PermissionDenied(permission)
        }
        RewardWalletCreditError::InvalidStatus(message) => {
            RewardReconciliationError::InvalidStatus(message)
        }
        RewardWalletCreditError::InvalidInput(message) => {
            RewardReconciliationError::InvalidInput(message)
        }
        RewardWalletCreditError::NoActivePolicy => RewardReconciliationError::NoActivePolicy,
        RewardWalletCreditError::NotFound => RewardReconciliationError::NotFound,
        RewardWalletCreditError::Connection(message)
        | RewardWalletCreditError::Database(message) => {
            RewardReconciliationError::Database(message)
        }
    }
}

pub(super) fn map_wallet_notification_error(
    error: RewardWalletCreditNotificationError,
) -> RewardReconciliationError {
    match error {
        RewardWalletCreditNotificationError::PermissionDenied(permission) => {
            RewardReconciliationError::PermissionDenied(permission)
        }
        RewardWalletCreditNotificationError::InvalidStatus(message) => {
            RewardReconciliationError::InvalidStatus(message)
        }
        RewardWalletCreditNotificationError::InvalidInput(message) => {
            RewardReconciliationError::InvalidInput(message)
        }
        RewardWalletCreditNotificationError::NotFound => RewardReconciliationError::NotFound,
        RewardWalletCreditNotificationError::Connection(message)
        | RewardWalletCreditNotificationError::Database(message) => {
            RewardReconciliationError::Database(message)
        }
    }
}

pub(super) fn map_transaction_error(
    error: RewardReconciliationTransactionError,
) -> RewardReconciliationError {
    match error {
        RewardReconciliationTransactionError::Application(error) => error,
        RewardReconciliationTransactionError::Diesel(error) => map_diesel_error(error),
    }
}
