use actix_web::http::StatusCode;

use crate::application::wallet::manage_token_tax::WalletTokenTaxError;
use crate::http::errors::ApiError;

pub(in crate::http::wallet) fn wallet_token_tax_error(error: WalletTokenTaxError) -> ApiError {
    match error {
        WalletTokenTaxError::Connection(_) => super::db_connection_failed(),
        WalletTokenTaxError::PermissionDenied => ApiError::new(
            StatusCode::FORBIDDEN,
            "wallet_tax_permission_denied",
            "User does not have wallet tax permission",
        ),
        WalletTokenTaxError::InvalidInput(message) => super::invalid_input(message),
        WalletTokenTaxError::PermissionCheck(message)
        | WalletTokenTaxError::TaxLoad(message)
        | WalletTokenTaxError::TaxStore(message) => super::logged_internal(
            "wallet_token_tax_api_failed",
            "Failed to process wallet token transfer",
            message,
        ),
    }
}
