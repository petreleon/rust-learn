use futures::future::BoxFuture;

use crate::application::wallet::retire_tokens::{
    WalletRetirementError, WalletRetirementRequest, WalletRetirementView,
};

pub trait WalletRetirementUseCase: Send + Sync {
    fn retire_tokens(
        &self,
        user_id: i32,
        request: WalletRetirementRequest,
    ) -> BoxFuture<'_, Result<WalletRetirementView, WalletRetirementError>>;
}
