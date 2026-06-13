use futures::future::BoxFuture;

use crate::application::wallet::read_wallet::{WalletReadError, WalletReadSubject, WalletView};

pub trait WalletReadUseCase: Send + Sync {
    fn read_wallet(
        &self,
        actor_user_id: i32,
        subject: WalletReadSubject,
    ) -> BoxFuture<'_, Result<WalletView, WalletReadError>>;
}
