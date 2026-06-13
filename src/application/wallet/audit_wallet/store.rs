use futures::future::BoxFuture;

use crate::application::wallet::audit_wallet::{WalletAudit, WalletAuditError, WalletAuditTarget};

pub trait WalletAuditStore {
    fn load_wallet_audit(
        &mut self,
        target: WalletAuditTarget,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>>;
}
