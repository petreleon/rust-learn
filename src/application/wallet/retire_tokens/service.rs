use futures::future::BoxFuture;

use crate::application::wallet::retire_tokens::{
    WalletRetirementCommand, WalletRetirementError, WalletRetirementView,
};

pub trait WalletRetirementUseCase: Send + Sync {
    fn retire_tokens(
        &self,
        user_id: i32,
        request: WalletRetirementCommand,
    ) -> BoxFuture<'_, Result<WalletRetirementView, WalletRetirementError>>;
}
