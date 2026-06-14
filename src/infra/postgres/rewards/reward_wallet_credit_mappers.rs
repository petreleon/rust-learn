use crate::application::rewards::credit_wallet::RewardWalletCreditError;
use crate::application::rewards::plan_payout::RewardPayoutPlanError;

pub(super) enum RewardWalletCreditTransactionError {
    Application(RewardWalletCreditError),
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for RewardWalletCreditTransactionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl From<RewardWalletCreditError> for RewardWalletCreditTransactionError {
    fn from(error: RewardWalletCreditError) -> Self {
        Self::Application(error)
    }
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> RewardWalletCreditError {
    match error {
        diesel::result::Error::NotFound => RewardWalletCreditError::NotFound,
        other => RewardWalletCreditError::Database(other.to_string()),
    }
}

pub(super) fn map_payout_plan_error(error: RewardPayoutPlanError) -> RewardWalletCreditError {
    match error {
        RewardPayoutPlanError::PermissionDenied(permission) => {
            RewardWalletCreditError::PermissionDenied(permission)
        }
        RewardPayoutPlanError::InvalidStatus(message) => {
            RewardWalletCreditError::InvalidStatus(message)
        }
        RewardPayoutPlanError::InvalidInput(message) => {
            RewardWalletCreditError::InvalidInput(message)
        }
        RewardPayoutPlanError::NoActivePolicy => RewardWalletCreditError::NoActivePolicy,
        RewardPayoutPlanError::Connection(message) | RewardPayoutPlanError::Database(message) => {
            RewardWalletCreditError::Database(message)
        }
    }
}

pub(super) fn map_transaction_error(
    error: RewardWalletCreditTransactionError,
) -> RewardWalletCreditError {
    match error {
        RewardWalletCreditTransactionError::Application(error) => error,
        RewardWalletCreditTransactionError::Diesel(error) => map_diesel_error(error),
    }
}
