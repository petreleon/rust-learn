use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::audit_wallet::{
    WalletAudit, WalletAuditError, WalletAuditStore, WalletAuditTarget, WalletAuditWallet,
};

pub(crate) struct FakeWalletAuditStore {
    pub loaded: bool,
    pub checked_user_permission: bool,
    pub checked_organization_permission: bool,
    pub checked_user_exists: bool,
    pub checked_organization_exists: bool,
    pub user_permission: bool,
    pub organization_exists: bool,
}

impl Default for FakeWalletAuditStore {
    fn default() -> Self {
        Self {
            loaded: false,
            checked_user_permission: false,
            checked_organization_permission: false,
            checked_user_exists: false,
            checked_organization_exists: false,
            user_permission: true,
            organization_exists: true,
        }
    }
}

impl WalletAuditStore for FakeWalletAuditStore {
    fn can_view_user_wallet(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        self.checked_user_permission = true;
        ready(Ok(self.user_permission)).boxed()
    }

    fn can_view_organization_wallet(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        self.checked_organization_permission = true;
        ready(Ok(true)).boxed()
    }

    fn user_exists(&mut self, _user_id: i32) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        self.checked_user_exists = true;
        ready(Ok(true)).boxed()
    }

    fn organization_exists(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        self.checked_organization_exists = true;
        ready(Ok(self.organization_exists)).boxed()
    }

    fn find_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletAuditTarget>, WalletAuditError>> {
        ready(Ok(Some(WalletAuditTarget {
            id: 10,
            user_id: Some(user_id),
            organization_id: None,
            value: "100".to_string(),
        })))
        .boxed()
    }

    fn find_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletAuditTarget>, WalletAuditError>> {
        ready(Ok(Some(WalletAuditTarget {
            id: 10,
            user_id: None,
            organization_id: Some(organization_id),
            value: "100".to_string(),
        })))
        .boxed()
    }

    fn load_wallet_audit(
        &mut self,
        target: WalletAuditTarget,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>> {
        self.loaded = true;
        ready(Ok(WalletAudit {
            wallet: WalletAuditWallet {
                id: target.id,
                owner_type: target.owner_type(),
                user_id: target.user_id,
                organization_id: target.organization_id,
                value: target.value,
            },
            internal_transactions: Vec::new(),
            external_transactions: Vec::new(),
            deposit_intents: Vec::new(),
            reward_records: Vec::new(),
            compensation_records: Vec::new(),
        }))
        .boxed()
    }
}
