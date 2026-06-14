use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::read_wallet::{WalletReadError, WalletReadStore, WalletView};

pub(crate) struct FakeWalletReadStore {
    pub checked_user_permission: bool,
    pub checked_organization_permission: bool,
    pub checked_user_exists: bool,
    pub checked_organization_exists: bool,
    pub user_permission: bool,
    pub organization_exists: bool,
}

impl Default for FakeWalletReadStore {
    fn default() -> Self {
        Self {
            checked_user_permission: false,
            checked_organization_permission: false,
            checked_user_exists: false,
            checked_organization_exists: false,
            user_permission: true,
            organization_exists: true,
        }
    }
}

impl WalletReadStore for FakeWalletReadStore {
    fn can_view_user_wallet(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        self.checked_user_permission = true;
        ready(Ok(self.user_permission)).boxed()
    }

    fn can_view_organization_wallet(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        self.checked_organization_permission = true;
        ready(Ok(true)).boxed()
    }

    fn user_exists(&mut self, _user_id: i32) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        self.checked_user_exists = true;
        ready(Ok(true)).boxed()
    }

    fn organization_exists(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        self.checked_organization_exists = true;
        ready(Ok(self.organization_exists)).boxed()
    }

    fn find_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletView>, WalletReadError>> {
        ready(Ok(Some(WalletView {
            id: 10,
            owner_type: "user",
            user_id: Some(user_id),
            organization_id: None,
            value: "100".to_string(),
        })))
        .boxed()
    }

    fn find_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletView>, WalletReadError>> {
        ready(Ok(Some(WalletView {
            id: 11,
            owner_type: "organization",
            user_id: None,
            organization_id: Some(organization_id),
            value: "200".to_string(),
        })))
        .boxed()
    }
}
