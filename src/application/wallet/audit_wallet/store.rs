use futures::future::BoxFuture;

use crate::application::wallet::audit_wallet::{WalletAudit, WalletAuditError, WalletAuditTarget};

pub trait WalletAuditStore {
    fn can_view_user_wallet(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>>;

    fn can_view_organization_wallet(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>>;

    fn user_exists(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletAuditError>>;

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>>;

    fn find_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletAuditTarget>, WalletAuditError>>;

    fn find_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletAuditTarget>, WalletAuditError>>;

    fn load_wallet_audit(
        &mut self,
        target: WalletAuditTarget,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>>;
}
