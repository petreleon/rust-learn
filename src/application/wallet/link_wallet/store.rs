use futures::future::BoxFuture;

use crate::application::wallet::link_wallet::{LinkedWalletView, WalletLinkError};

pub trait WalletLinkStore {
    fn can_link_user_wallet(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>>;

    fn can_link_organization_wallet(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>>;

    fn user_exists(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletLinkError>>;

    fn user_kyc_verified(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletLinkError>>;

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>>;

    fn link_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>>;

    fn link_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>>;
}
