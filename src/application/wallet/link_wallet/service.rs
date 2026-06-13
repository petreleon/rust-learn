use futures::future::BoxFuture;

use crate::application::wallet::link_wallet::{
    LinkedWalletView, WalletLinkError, WalletLinkSubject,
};

pub trait WalletLinkUseCase: Send + Sync {
    fn link_wallet(
        &self,
        actor_user_id: i32,
        subject: WalletLinkSubject,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>>;
}
