use actix_web::http::StatusCode;

use crate::application::wallet::create_deposit_intent::WalletDepositIntentError;
use crate::application::wallet::retire_tokens::WalletRetirementError;
use crate::http::errors::ApiError;

pub(in crate::http::wallet) fn wallet_deposit_intent_error(
    error: WalletDepositIntentError,
) -> ApiError {
    match error {
        WalletDepositIntentError::Connection(_) => super::db_connection_failed(),
        WalletDepositIntentError::KycRequired => super::kyc_required(),
        WalletDepositIntentError::InvalidInput(message) => super::invalid_input(message),
        WalletDepositIntentError::KycLoad(message)
        | WalletDepositIntentError::TaxLoad(message)
        | WalletDepositIntentError::ConfigurationLoad(message)
        | WalletDepositIntentError::WalletLoad(message)
        | WalletDepositIntentError::WalletCreate(message)
        | WalletDepositIntentError::DepositIntentCreate(message) => super::logged_internal(
            "wallet_deposit_intent_api_failed",
            "Failed to process wallet token transfer",
            message,
        ),
    }
}

pub(in crate::http::wallet) fn wallet_retirement_error(error: WalletRetirementError) -> ApiError {
    match error {
        WalletRetirementError::Connection(_) => super::db_connection_failed(),
        WalletRetirementError::KycRequired => super::kyc_required(),
        WalletRetirementError::InvalidInput(message) => super::invalid_input(message),
        WalletRetirementError::InsufficientFunds => ApiError::new(
            StatusCode::CONFLICT,
            "insufficient_funds",
            "Insufficient wallet balance",
        ),
        WalletRetirementError::KycLoad(message)
        | WalletRetirementError::TaxLoad(message)
        | WalletRetirementError::WalletLoad(message)
        | WalletRetirementError::WalletCreate(message)
        | WalletRetirementError::RetirementCreate(message) => super::logged_internal(
            "wallet_retirement_api_failed",
            "Failed to process wallet token transfer",
            message,
        ),
    }
}
