use futures::future::BoxFuture;

use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationAction, WalletAuthorizationError,
};

pub trait WalletAuthorizationUseCase: Send + Sync {
    fn authorize_wallet_action(
        &self,
        actor_user_id: i32,
        action: WalletAuthorizationAction,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>>;
}
