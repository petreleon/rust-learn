use actix_web::http::StatusCode;

use crate::application::wallet::burn_tokens::TokenBurnError;
use crate::http::errors::ApiError;

pub(in crate::http::wallet) fn token_burn_error(error: TokenBurnError) -> ApiError {
    match error {
        TokenBurnError::Connection(_) => super::db_connection_failed(),
        TokenBurnError::KycRequired => super::kyc_required(),
        TokenBurnError::PermissionDenied => ApiError::new(
            StatusCode::FORBIDDEN,
            "token_burn_permission_denied",
            "User does not have token burn permission",
        ),
        TokenBurnError::OrganizationNotFound => super::organization_not_found(),
        TokenBurnError::BurnNotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "token_burn_not_found",
            "Token burn request was not found",
        ),
        TokenBurnError::InvalidInput(message) => super::invalid_input(message),
        TokenBurnError::InsufficientFunds => ApiError::new(
            StatusCode::CONFLICT,
            "insufficient_funds",
            "Insufficient wallet balance",
        ),
        TokenBurnError::KycLoad(message)
        | TokenBurnError::PermissionCheck(message)
        | TokenBurnError::OrganizationLoad(message)
        | TokenBurnError::TaxLoad(message)
        | TokenBurnError::WalletLoad(message)
        | TokenBurnError::WalletCreate(message)
        | TokenBurnError::BurnCreate(message)
        | TokenBurnError::BurnLoad(message)
        | TokenBurnError::BurnReconcile(message)
        | TokenBurnError::LeaderboardLoad(message) => super::logged_internal(
            "token_burn_api_failed",
            "Failed to process token burn",
            message,
        ),
    }
}
