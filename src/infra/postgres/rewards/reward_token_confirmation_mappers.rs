use crate::application::rewards::record_token_confirmation::RewardTokenConfirmationError;

pub(super) enum RewardTokenConfirmationTransactionError {
    Application(RewardTokenConfirmationError),
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for RewardTokenConfirmationTransactionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl From<RewardTokenConfirmationError> for RewardTokenConfirmationTransactionError {
    fn from(error: RewardTokenConfirmationError) -> Self {
        Self::Application(error)
    }
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> RewardTokenConfirmationError {
    match error {
        diesel::result::Error::NotFound => RewardTokenConfirmationError::NotFound,
        other => RewardTokenConfirmationError::Database(other.to_string()),
    }
}

pub(super) fn map_transaction_error(
    error: RewardTokenConfirmationTransactionError,
) -> RewardTokenConfirmationError {
    match error {
        RewardTokenConfirmationTransactionError::Application(error) => error,
        RewardTokenConfirmationTransactionError::Diesel(error) => map_diesel_error(error),
    }
}
