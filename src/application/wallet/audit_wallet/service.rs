use futures::future::BoxFuture;

use crate::application::wallet::audit_wallet::{WalletAudit, WalletAuditError, WalletAuditTarget};

pub trait WalletAuditUseCase: Send + Sync {
    fn audit_wallet(
        &self,
        target: WalletAuditTarget,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>>;
}
