use futures::future::BoxFuture;

use crate::application::wallet::read_wallet::{WalletReadError, WalletView};

pub trait WalletReadStore {
    fn can_view_user_wallet(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>>;

    fn can_view_organization_wallet(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>>;

    fn user_exists(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletReadError>>;

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>>;

    fn find_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletView>, WalletReadError>>;

    fn find_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletView>, WalletReadError>>;
}
