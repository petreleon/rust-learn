use futures::future::BoxFuture;

use crate::application::wallet::create_deposit_intent::{
    WalletDepositIntentCommand, WalletDepositIntentError, WalletDepositIntentView,
};

pub trait WalletDepositIntentUseCase: Send + Sync {
    fn create_deposit_intent(
        &self,
        user_id: i32,
        request: WalletDepositIntentCommand,
    ) -> BoxFuture<'_, Result<WalletDepositIntentView, WalletDepositIntentError>>;
}
