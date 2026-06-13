use futures::future::BoxFuture;

use crate::application::wallet::audit_wallet::{WalletAudit, WalletAuditError, WalletAuditSubject};

pub trait WalletAuditUseCase: Send + Sync {
    fn audit_wallet(
        &self,
        actor_user_id: i32,
        subject: WalletAuditSubject,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>>;
}
