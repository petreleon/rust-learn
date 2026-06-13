use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationError;

pub(super) enum RewardWalletCreditNotificationTransactionError {
    Application(RewardWalletCreditNotificationError),
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for RewardWalletCreditNotificationTransactionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl From<RewardWalletCreditNotificationError> for RewardWalletCreditNotificationTransactionError {
    fn from(error: RewardWalletCreditNotificationError) -> Self {
        Self::Application(error)
    }
}

pub(super) fn map_diesel_error(
    error: diesel::result::Error,
) -> RewardWalletCreditNotificationError {
    match error {
        diesel::result::Error::NotFound => RewardWalletCreditNotificationError::NotFound,
        other => RewardWalletCreditNotificationError::Database(other.to_string()),
    }
}

pub(super) fn map_transaction_error(
    error: RewardWalletCreditNotificationTransactionError,
) -> RewardWalletCreditNotificationError {
    match error {
        RewardWalletCreditNotificationTransactionError::Application(error) => error,
        RewardWalletCreditNotificationTransactionError::Diesel(error) => map_diesel_error(error),
    }
}
