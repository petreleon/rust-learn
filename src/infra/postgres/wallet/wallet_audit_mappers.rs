use crate::application::wallet::audit_wallet::{WalletAuditError, WalletAuditTarget};
use crate::models::wallet::Wallet;

pub fn wallet_audit_target_from_model(wallet: &Wallet) -> WalletAuditTarget {
    WalletAuditTarget {
        id: wallet.id,
        user_id: wallet.user_id,
        organization_id: wallet.organization_id,
        value: wallet.value.to_string(),
    }
}

pub(super) fn map_wallet_audit_error(error: diesel::result::Error) -> WalletAuditError {
    WalletAuditError::Database(error.to_string())
}
