use actix_web::http::StatusCode;

use crate::http::errors::ApiError;

mod audit;
mod link;
mod read;
mod tax;
mod transfer;

pub(super) use audit::wallet_audit_error;
pub(super) use link::wallet_link_error;
pub(super) use read::wallet_read_error;
pub(super) use tax::wallet_token_tax_error;
pub(super) use transfer::{wallet_deposit_intent_error, wallet_retirement_error};

pub(in crate::http::wallet) fn db_connection_failed() -> ApiError {
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "db_connection_failed",
        "Failed to get DB connection",
    )
}

pub(in crate::http::wallet) fn user_not_found() -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND, "user_not_found", "User not found")
}

pub(in crate::http::wallet) fn organization_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "organization_not_found",
        "Organization not found",
    )
}

pub(in crate::http::wallet) fn wallet_not_linked() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "wallet_not_linked",
        "Wallet not linked",
    )
}

pub(in crate::http::wallet) fn kyc_required() -> ApiError {
    ApiError::new(
        StatusCode::CONFLICT,
        "kyc_required",
        "KYC verification is required before wallet actions",
    )
}

pub(in crate::http::wallet) fn user_wallet_access_denied() -> ApiError {
    ApiError::new(
        StatusCode::FORBIDDEN,
        "wallet_access_denied",
        "User does not have wallet access",
    )
}

pub(in crate::http::wallet) fn organization_wallet_access_denied() -> ApiError {
    ApiError::new(
        StatusCode::FORBIDDEN,
        "organization_wallet_access_denied",
        "User does not have organization wallet access",
    )
}

pub(in crate::http::wallet) fn invalid_input(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, "invalid_input", message)
}

pub(in crate::http::wallet) fn logged_internal(
    event: &'static str,
    message: &'static str,
    error: String,
) -> ApiError {
    log::error!("event={} error={}", event, error);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "wallet_request_failed",
        message,
    )
}

#[cfg(test)]
mod tests;
