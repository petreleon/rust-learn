use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationError;

pub(super) fn map_diesel_error(error: diesel::result::Error) -> PlatformWalletReconciliationError {
    PlatformWalletReconciliationError::Database(error.to_string())
}
