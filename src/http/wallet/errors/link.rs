use crate::application::wallet::link_wallet::WalletLinkError;
use crate::http::errors::ApiError;

pub(in crate::http::wallet) fn wallet_link_error(error: WalletLinkError) -> ApiError {
    match error {
        WalletLinkError::Connection(_) => super::db_connection_failed(),
        WalletLinkError::UserNotFound => super::user_not_found(),
        WalletLinkError::OrganizationNotFound => super::organization_not_found(),
        WalletLinkError::KycRequired => super::kyc_required(),
        WalletLinkError::UserPermissionDenied => super::user_wallet_access_denied(),
        WalletLinkError::OrganizationPermissionDenied => super::organization_wallet_access_denied(),
        WalletLinkError::UserLoad(message) => super::logged_internal(
            "wallet_link_user_load_failed",
            "Failed to load user",
            message,
        ),
        WalletLinkError::OrganizationLoad(message) => super::logged_internal(
            "wallet_link_organization_load_failed",
            "Failed to load organization",
            message,
        ),
        WalletLinkError::UserAccessCheck(message) => super::logged_internal(
            "wallet_link_user_access_failed",
            "Failed to check wallet access",
            message,
        ),
        WalletLinkError::OrganizationAccessCheck(message) => super::logged_internal(
            "wallet_link_organization_access_failed",
            "Failed to check organization wallet access",
            message,
        ),
        WalletLinkError::KycLoad(message) => super::logged_internal(
            "wallet_link_kyc_load_failed",
            "Failed to load user KYC status",
            message,
        ),
        WalletLinkError::WalletLoad(message) | WalletLinkError::WalletCreate(message) => {
            super::logged_internal("wallet_link_failed", "Failed to link wallet", message)
        }
    }
}
