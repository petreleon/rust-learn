use futures::future::BoxFuture;

use crate::application::access_control::authorize_wallet::WalletAuthorizationError;
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};

pub trait WalletAuthorizationStore {
    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>>;
}
