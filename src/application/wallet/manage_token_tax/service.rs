use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::wallet::manage_token_tax::{
    WalletTokenTaxAuditEventView, WalletTokenTaxError, WalletTokenTaxOperation,
    WalletTokenTaxSettings, WalletTokenTaxView,
};

pub trait WalletTokenTaxUseCase: Send + Sync {
    fn list_token_taxes(
        &self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<WalletTokenTaxSettings, WalletTokenTaxError>>;

    fn list_token_tax_audit(
        &self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<WalletTokenTaxAuditEventView>, WalletTokenTaxError>>;

    fn set_token_tax(
        &self,
        actor_user_id: i32,
        operation: WalletTokenTaxOperation,
        amount: BigDecimal,
    ) -> BoxFuture<'_, Result<WalletTokenTaxView, WalletTokenTaxError>>;
}
