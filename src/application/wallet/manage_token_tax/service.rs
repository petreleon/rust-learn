use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxError, WalletTokenTaxOperation, WalletTokenTaxSettings, WalletTokenTaxView,
};

pub trait WalletTokenTaxUseCase: Send + Sync {
    fn list_token_taxes(
        &self,
    ) -> BoxFuture<'_, Result<WalletTokenTaxSettings, WalletTokenTaxError>>;

    fn set_token_tax(
        &self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<WalletTokenTaxView, WalletTokenTaxError>>;
}
