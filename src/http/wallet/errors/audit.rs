use crate::application::wallet::audit_wallet::WalletAuditError;
use crate::http::errors::ApiError;

pub(in crate::http::wallet) fn wallet_audit_error(error: WalletAuditError) -> ApiError {
    match error {
        WalletAuditError::Connection(_) => super::db_connection_failed(),
        WalletAuditError::UserNotFound => super::user_not_found(),
        WalletAuditError::OrganizationNotFound => super::organization_not_found(),
        WalletAuditError::WalletNotLinked => super::wallet_not_linked(),
        WalletAuditError::UserPermissionDenied => super::user_wallet_access_denied(),
        WalletAuditError::OrganizationPermissionDenied => {
            super::organization_wallet_access_denied()
        }
        WalletAuditError::UserLoad(message) => super::logged_internal(
            "wallet_audit_user_load_failed",
            "Failed to load user",
            message,
        ),
        WalletAuditError::OrganizationLoad(message) => super::logged_internal(
            "wallet_audit_organization_load_failed",
            "Failed to load organization",
            message,
        ),
        WalletAuditError::UserAccessCheck(message) => super::logged_internal(
            "wallet_audit_user_access_failed",
            "Failed to check wallet access",
            message,
        ),
        WalletAuditError::OrganizationAccessCheck(message) => super::logged_internal(
            "wallet_audit_organization_access_failed",
            "Failed to check organization wallet access",
            message,
        ),
        WalletAuditError::WalletLoad(message) => super::logged_internal(
            "wallet_audit_wallet_load_failed",
            "Failed to load wallet",
            message,
        ),
        WalletAuditError::AuditLoad(message) | WalletAuditError::Database(message) => {
            super::logged_internal(
                "wallet_audit_load_failed",
                "Failed to load wallet audit",
                message,
            )
        }
    }
}
