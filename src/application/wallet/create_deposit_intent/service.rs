use futures::future::BoxFuture;

use crate::application::wallet::create_deposit_intent::{
    WalletDepositIntentError, WalletDepositIntentRequest, WalletDepositIntentView,
};

pub trait WalletDepositIntentUseCase: Send + Sync {
    fn create_deposit_intent(
        &self,
        user_id: i32,
        request: WalletDepositIntentRequest,
    ) -> BoxFuture<'_, Result<WalletDepositIntentView, WalletDepositIntentError>>;
}
