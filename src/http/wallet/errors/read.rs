use crate::application::wallet::read_wallet::WalletReadError;
use crate::http::errors::ApiError;

pub(in crate::http::wallet) fn wallet_read_error(error: WalletReadError) -> ApiError {
    match error {
        WalletReadError::Connection(_) => super::db_connection_failed(),
        WalletReadError::UserNotFound => super::user_not_found(),
        WalletReadError::OrganizationNotFound => super::organization_not_found(),
        WalletReadError::WalletNotLinked => super::wallet_not_linked(),
        WalletReadError::UserPermissionDenied => super::user_wallet_access_denied(),
        WalletReadError::OrganizationPermissionDenied => super::organization_wallet_access_denied(),
        WalletReadError::UserLoad(message) => super::logged_internal(
            "wallet_read_user_load_failed",
            "Failed to load user",
            message,
        ),
        WalletReadError::OrganizationLoad(message) => super::logged_internal(
            "wallet_read_organization_load_failed",
            "Failed to load organization",
            message,
        ),
        WalletReadError::UserAccessCheck(message) => super::logged_internal(
            "wallet_read_user_access_failed",
            "Failed to check wallet access",
            message,
        ),
        WalletReadError::OrganizationAccessCheck(message) => super::logged_internal(
            "wallet_read_organization_access_failed",
            "Failed to check organization wallet access",
            message,
        ),
        WalletReadError::WalletLoad(message) => super::logged_internal(
            "wallet_read_wallet_load_failed",
            "Failed to load wallet",
            message,
        ),
    }
}
