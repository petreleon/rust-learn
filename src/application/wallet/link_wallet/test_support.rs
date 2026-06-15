use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::wallet::link_wallet::{LinkedWalletView, WalletLinkError, WalletLinkStore};
use crate::application::wallet::wallet_view::WalletView;
use crate::domain::wallet::owner::WalletOwnerType;

pub(crate) struct FakeWalletLinkStore {
    pub checked_user_permission: bool,
    pub checked_organization_permission: bool,
    pub checked_user_exists: bool,
    pub checked_user_kyc: bool,
    pub checked_organization_exists: bool,
    pub linked_user_wallet: bool,
    pub linked_organization_wallet: bool,
    pub user_permission: bool,
    pub organization_permission: bool,
    pub user_exists: bool,
    pub user_kyc_verified: bool,
    pub organization_exists: bool,
}

impl Default for FakeWalletLinkStore {
    fn default() -> Self {
        Self {
            checked_user_permission: false,
            checked_organization_permission: false,
            checked_user_exists: false,
            checked_user_kyc: false,
            checked_organization_exists: false,
            linked_user_wallet: false,
            linked_organization_wallet: false,
            user_permission: true,
            organization_permission: true,
            user_exists: true,
            user_kyc_verified: true,
            organization_exists: true,
        }
    }
}

impl WalletLinkStore for FakeWalletLinkStore {
    fn can_link_user_wallet(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        self.checked_user_permission = true;
        ready(Ok(self.user_permission)).boxed()
    }

    fn can_link_organization_wallet(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        self.checked_organization_permission = true;
        ready(Ok(self.organization_permission)).boxed()
    }

    fn user_exists(&mut self, _user_id: i32) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        self.checked_user_exists = true;
        ready(Ok(self.user_exists)).boxed()
    }

    fn user_kyc_verified(&mut self, _user_id: i32) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        self.checked_user_kyc = true;
        ready(Ok(self.user_kyc_verified)).boxed()
    }

    fn organization_exists(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        self.checked_organization_exists = true;
        ready(Ok(self.organization_exists)).boxed()
    }

    fn link_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>> {
        self.linked_user_wallet = true;
        ready(Ok(LinkedWalletView {
            wallet: WalletView {
                id: 10,
                owner_type: WalletOwnerType::User,
                user_id: Some(user_id),
                organization_id: None,
                value: "0".to_string(),
            },
            created: true,
        }))
        .boxed()
    }

    fn link_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>> {
        self.linked_organization_wallet = true;
        ready(Ok(LinkedWalletView {
            wallet: WalletView {
                id: 11,
                owner_type: WalletOwnerType::Organization,
                user_id: None,
                organization_id: Some(organization_id),
                value: "0".to_string(),
            },
            created: true,
        }))
        .boxed()
    }
}
