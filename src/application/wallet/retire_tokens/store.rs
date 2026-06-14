use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::wallet::retire_tokens::{
    WalletRetirementDraft, WalletRetirementError, WalletRetirementView,
};

pub trait WalletRetirementStore {
    fn user_kyc_verified(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletRetirementError>>;

    fn load_platform_retire_tax(
        &mut self,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletRetirementError>>;

    fn retire_tokens(
        &mut self,
        user_id: i32,
        draft: WalletRetirementDraft,
    ) -> BoxFuture<'_, Result<WalletRetirementView, WalletRetirementError>>;
}
